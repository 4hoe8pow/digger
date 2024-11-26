use crate::TicketCollection;
use color_eyre::Result;
use ddomain::domain_errors::DomainError;
use ddomain::entites::ticket::Ticket;
use ddomain::repositories::ticket_repository::TicketRepository;
use serde::Deserialize;
use std::fs;
use std::sync::{Arc, RwLock};
use toml;

pub struct TicketRepositoryImpl {
    file_path: String,
    ticket_cache: Arc<RwLock<Vec<Ticket>>>,
}

impl TicketRepositoryImpl {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
            ticket_cache: Arc::new(RwLock::new(Vec::new())),
        }
    }

    fn deserial_toml_file<T>(&self) -> Result<T, DomainError>
    where
        T: for<'a> Deserialize<'a>,
    {
        let file_str = fs::read_to_string(&self.file_path).map_err(DomainError::File)?;
        if file_str.trim().is_empty() {
            Err(DomainError::EmptyFile)
        } else {
            toml::from_str(&file_str).map_err(DomainError::TomlParse)
        }
    }

    fn load_tickets_from_file(&self) -> Result<Vec<Ticket>, DomainError> {
        let ticket_collection: TicketCollection = self.deserial_toml_file::<TicketCollection>()?;
        Ok(ticket_collection.ticket_data)
    }

    pub fn count_tickets(&self) -> Result<usize, DomainError> {
        let cache = self.ticket_cache.read().unwrap();
        if cache.is_empty() {
            // キャッシュが空ならファイルから読み込んでその件数を返す
            drop(cache);
            let tickets = self.load_tickets_from_file()?;
            Ok(tickets.len())
        } else {
            // キャッシュにデータがあれば、キャッシュの件数を返す
            Ok(cache.len())
        }
    }

    fn update_tickets_in_file(&self, tickets: &[Ticket]) -> Result<(), DomainError> {
        let ticket_collection = TicketCollection {
            ticket_data: tickets.to_vec(),
        };
        let serialized = toml::to_string(&ticket_collection).map_err(DomainError::TomlSerialize)?;
        fs::write(&self.file_path, serialized).map_err(DomainError::File)?;
        Ok(())
    }
}

impl TicketRepository for TicketRepositoryImpl {
    fn fetch_tickets(&self) -> Result<Vec<Ticket>, DomainError> {
        // キャッシュが空でないかチェック
        let cache = self.ticket_cache.read().unwrap();
        if cache.is_empty() {
            // キャッシュが空の場合、ファイルから読み込んでキャッシュに保存
            drop(cache);
            let tickets = self.load_tickets_from_file()?;
            let mut cache = self.ticket_cache.write().unwrap();
            *cache = tickets.clone(); // チケットをキャッシュに保存
            Ok(tickets)
        } else {
            Ok(cache.clone())
        }
    }

    fn save(&mut self, update_ticket: Ticket) -> Result<(), DomainError> {
        let mut cache = self.ticket_cache.write().unwrap();

        // Find and update the ticket, or return an error if not found
        cache
            .iter_mut()
            .find(|ticket| **ticket == update_ticket)
            .map(|ticket| *ticket = update_ticket)
            .ok_or(DomainError::TicketNotFound("Ticket not found".to_string()))?;

        // Write the updated cache to the file
        self.update_tickets_in_file(&cache)?;

        Ok(())
    }
}
