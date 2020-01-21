use pakistan;
mod lib;
mod human_resources{
    pub mod software_developer_attendance{
pub fn time(){
    println!("We are in humanResource module::softwareDeveloperAttendance::time");
}
}
    }

//use crate::human_resources::software_developer_attendance;
//use crate::officals :: staff::member;
fn main() {
    println!("Hello, world!");
    pakistan::islamabad::piaic();

//***Assignment7 question#1*****
//Relative Path
human_resources::software_developer_attendance::time();
//For question #2
//crate::officals ::staff::member ::int();
crate::lib::officals::staff::member ::int();
//member::int(); 
//lib::int();
///*************question#3******
hr_table::food::kfc::burger();
}
