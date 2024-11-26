use crate::value_objects::ticket_level::TicketLevel;
use crate::value_objects::ticket_status::TicketStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Ticket {
    id: String,
    level: TicketLevel,
    title: String,
    status: TicketStatus,
    created_at: DateTime<Utc>,
    resolved_at: Option<DateTime<Utc>>,
    completion_condition: String,
}

impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Serialize)]
pub struct TicketMut<'a> {
    pub id: &'a mut String,
    pub level: &'a mut TicketLevel,
    pub title: &'a mut String,
    pub status: &'a mut TicketStatus,
    pub created_at: &'a mut DateTime<Utc>,
    pub resolved_at: &'a mut Option<DateTime<Utc>>,
    pub completion_condition: &'a mut String,
}

impl Ticket {
    pub fn new(
        id: String,
        level: TicketLevel,
        title: String,
        status: TicketStatus,
        completion_condition: String,
    ) -> Self {
        Ticket {
            id,
            level,
            title,
            status,
            created_at: Utc::now(),
            resolved_at: None,
            completion_condition,
        }
    }

    pub fn substitute(&mut self, f: impl FnOnce(&mut TicketMut)) {
        f(&mut TicketMut {
            id: &mut self.id,
            level: &mut self.level,
            title: &mut self.title,
            status: &mut self.status,
            created_at: &mut self.created_at,
            resolved_at: &mut self.resolved_at,
            completion_condition: &mut self.completion_condition,
        });
    }

    pub fn set_status(&mut self, new_status: TicketStatus) {
        if new_status == TicketStatus::Resolved {
            // 解決日時を現在のUTC時間に設定
            self.resolved_at = Some(Utc::now());
        }
        self.status = new_status;
    }

    pub fn set_completion_condition(&mut self, new_condition: String) {
        self.completion_condition = new_condition;
    }
}
