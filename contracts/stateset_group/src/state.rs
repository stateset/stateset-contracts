use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Person 
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Person {
    pub name: String,
    pub membership_ids: Vec<String>
}

pub const PEOPLE: Map<&[u8], Person> = Map::new("people");


// Group
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Group {
    pub name: String,
    pub membership_ids: Vec<String>
}

pub const GROUPS: Map<&[u8], Group> = Map::new("groups");


// Membership
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Membership {
  pub person_id: String,
  pub group_id: String,
  pub membership_status_id: String
}

pub const MEMBERSHIPS: Map<&[u8], Membership> = Map::new("memberships");

// Membership Status
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MembershipStatus {
  pub status: String,
  pub membership_ids: Vec<String>
}

pub const MEMBERSHIP_STATUSES: Map<&[u8], MembershipStatus> = Map::new("membership_statuses");