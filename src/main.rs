extern crate chrono;
extern crate env_logger;
extern crate failure;
extern crate heck;
#[macro_use]
extern crate log;
extern crate names;
extern crate rand;
extern crate serde;
extern crate serde_derive;
extern crate shrinkwraprs;

use chrono::{DateTime, Duration, Utc};

use heck::TitleCase;

use failure::Error;

use shrinkwraprs::Shrinkwrap;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Result<T> = ::std::result::Result<T, Error>;

const NURSE_NAMES: &[&str] = &[
    "Nurse Ratched",
    "Florence Nightingale",
    "Nurse Cramer",
    "Poppy Pomfrey",
    "Laverne Roberts",
    "Christine Chapel",
    "Nurse Duckett",
    "Carla Espinoza",
    "Abby Lockhart",
    "Samantha Taggart",
];

const PATIENT_NAMES: &[&str] = &[
    "Don Young",
    "Bradley Byrne",
    "Martha Roby",
    "Mike Rogers",
    "Robert Aderholt",
    "Mo Brooks",
    "Gary Palmer",
    "Terri Sewell",
    "Rick Crawford",
    "French Hill",
    "Steve Womack",
    "Bruce Westerman",
    "Tom O'Halleran",
    "Martha McSally",
    "Raul Grijalva",
    "Paul Gosar",
    "Andy Biggs",
    "David Schweikert",
    "Ruben Gallego",
    "Trent Franks",
    "Kyrsten Sinema",
    "Doug LaMalfa",
    "Jared Huffman",
    "John Garamendi",
    "Tom McClintock",
    "Mike Thompson",
    "Doris Matsui",
    "Ami Bera",
    "Paul Cook",
    "Jerry McNerney",
    "Jeff Denham",
    "Mark DeSaulnier",
    "Nancy Pelosi",
    "Barbara Lee",
    "Jackie Speier",
    "Eric Swalwell",
    "Jim Costa",
    "Ro Khanna",
    "Anna Eshoo",
    "Zoe Lofgren",
    "Jimmy Panetta",
    "David Valadao",
    "Devin Nunes",
    "Kevin McCarthy",
    "Salud Carbajal",
    "Steve Knight",
    "Julia Brownley",
    "Judy Chu",
    "Adam Schiff",
    "Tony Cardenas",
    "Brad Sherman",
    "Pete Aguilar",
    "Grace Napolitano",
    "Ted Lieu",
    "Jimmy Gomez",
    "Norma Torres",
    "Raul Ruiz",
    "Karen Bass",
    "Linda Sánchez",
    "Ed Royce",
    "Lucille Roybal-Allard",
    "Mark Takano",
    "Ken Calvert",
    "Maxine Waters",
    "Nanette Barragan",
    "Mimi Walters",
    "Luis Correa",
    "Alan Lowenthal",
    "Dana Rohrabacher",
    "Darrell Issa",
    "Duncan Hunter",
    "Juan Vargas",
    "Scott Peters",
    "Susan Davis",
    "Diana DeGette",
    "Jared Polis",
    "Scott Tipton",
    "Ken Buck",
    "Doug Lamborn",
    "Mike Coffman",
    "Ed Perlmutter",
    "John Larson",
    "Joe Courtney",
    "Rosa DeLauro",
    "Jim Himes",
    "Elizabeth Esty",
    "Lisa Blunt Rochester",
    "Matt Gaetz",
    "Neal Dunn",
    "Ted Yoho",
    "John Rutherford",
    "Al Lawson",
    "Ron DeSantis",
    "Stephanie Murphy",
    "Bill Posey",
    "Darren Soto",
    "Val Demings",
    "Daniel Webster",
    "Gus Bilirakis",
    "Charlie Crist",
    "Kathy Castor",
    "Dennis Ross",
    "Vern Buchanan",
    "Tom Rooney",
    "Brian Mast",
    "Francis Rooney",
    "Alcee Hastings",
    "Lois Frankel",
    "Ted Deutch",
    "Debbie Wasserman Schultz",
    "Frederica Wilson",
    "Mario Diaz-Balart",
    "Carlos Curbelo",
    "Ileana Ros-Lehtinen",
    "Buddy Carter",
    "Sanford Bishop",
    "Drew Ferguson",
    "Henry Johnson",
    "John Lewis",
    "Karen Handel",
    "Robert Woodall",
    "Austin Scott",
    "Doug Collins",
    "Jody Hice",
    "Barry Loudermilk",
    "Rick Allen",
    "David Scott",
    "Tom Graves",
    "Colleen Hanabusa",
    "Tulsi Gabbard",
    "Rod Blum",
    "David Loebsack",
    "David Young",
    "Steve King",
    "Raúl Labrador",
    "Mike Simpson",
    "Bobby Rush",
    "Robin Kelly",
    "Daniel Lipinski",
    "Luis Gutiérrez",
    "Mike Quigley",
    "Peter Roskam",
    "Danny Davis",
    "Raja Krishnamoorthi",
    "Janice Schakowsky",
    "Bradley Schneider",
    "Bill Foster",
    "Mike Bost",
    "Rodney Davis",
    "Randy Hultgren",
    "John Shimkus",
    "Adam Kinzinger",
    "Cheri Bustos",
    "Darin LaHood",
    "Pete Visclosky",
    "Jackie Walorski",
    "Jim Banks",
    "Todd Rokita",
    "Susan Brooks",
    "Luke Messer",
    "André Carson",
    "Larry Bucshon",
    "Trey Hollingsworth",
    "Roger Marshall",
    "Lynn Jenkins",
    "Kevin Yoder",
    "Ron Estes",
    "James Comer",
    "Brett Guthrie",
    "John Yarmuth",
    "Thomas Massie",
    "Harold Rogers",
    "Andy Barr",
    "Steve Scalise",
    "Cedric Richmond",
    "Clay Higgins",
    "Mike Johnson",
    "Ralph Abraham",
    "Garret Graves",
    "Richard Neal",
    "James McGovern",
    "Niki Tsongas",
    "Joe Kennedy",
    "Katherine Clark",
    "Seth Moulton",
    "Michael Capuano",
    "Stephen Lynch",
    "William Keating",
    "Andy Harris",
    "Dutch Ruppersberger",
    "John Sarbanes",
    "Anthony Brown",
    "Steny Hoyer",
    "John Delaney",
    "Elijah Cummings",
    "Jamie Raskin",
    "Chellie Pingree",
    "Bruce Poliquin",
    "Jack Bergman",
    "Bill Huizenga",
    "Justin Amash",
    "John Moolenaar",
    "Daniel Kildee",
    "Fred Upton",
    "Tim Walberg",
    "Mike Bishop",
    "Sandy Levin",
    "Paul Mitchell",
    "Dave Trott",
    "Debbie Dingell",
    "John Conyers",
    "Brenda Lawrence",
    "Timothy Walz",
    "Jason Lewis",
    "Erik Paulsen",
    "Betty McCollum",
    "Keith Ellison",
    "Tom Emmer",
    "Collin Peterson",
    "Richard Nolan",
    "William Clay",
    "Ann Wagner",
    "Blaine Luetkemeyer",
    "Vicky Hartzler",
    "Emanuel Cleaver",
    "Sam Graves",
    "Billy Long",
    "Jason Smith",
    "Trent Kelly",
    "Bennie Thompson",
    "Gregg Harper",
    "Steven Palazzo",
    "Ryan Zinke",
    "G.K. Butterfield",
    "George Holding",
    "Walter Jones",
    "David Price",
    "Virginia Foxx",
    "Mark Walker",
    "David Rouzer",
    "Richard Hudson",
    "Robert Pittenger",
    "Patrick McHenry",
    "Mark Meadows",
    "Alma Adams",
    "Ted Budd",
    "Kevin Cramer",
    "Jeff Fortenberry",
    "Don Bacon",
    "Adrian Smith",
    "Carol Shea-Porter",
    "Annie Kuster",
    "Donald Norcross",
    "Frank LoBiondo",
    "Tom MacArthur",
    "Christopher Smith",
    "Josh Gottheimer",
    "Frank Pallone",
    "Leonard Lance",
    "Albio Sires",
    "Bill Pascrell",
    "Donald Payne",
    "Rodney Frelinghuysen",
    "Bonnie Watson Coleman",
    "Michelle Lujan Grisham",
    "Steve Pearce",
    "Ben Ray Luján",
    "Dina Titus",
    "Mark Amodei",
    "Jacky Rosen",
    "Ruben Kihuen",
    "Lee Zeldin",
    "Pete King",
    "Thomas Suozzi",
    "Kathleen Rice",
    "Gregory Meeks",
    "Grace Meng",
    "Nydia Velázquez",
    "Hakeem Jeffries",
    "Yvette Clarke",
    "Jerrold Nadler",
    "Daniel Donovan",
    "Carolyn Maloney",
    "Adriano Espaillat",
    "Joseph Crowley",
    "José Serrano",
    "Eliot Engel",
    "Nita Lowey",
    "Sean Patrick Maloney",
    "John Faso",
    "Paul Tonko",
    "Elise Stefanik",
    "Claudia Tenney",
    "Tom Reed",
    "John Katko",
    "Louise Slaughter",
    "Brian Higgins",
    "Chris Collins",
    "Steve Chabot",
    "Brad Wenstrup",
    "Joyce Beatty",
    "Jim Jordan",
    "Robert Latta",
    "Bill Johnson",
    "Bob Gibbs",
    "Warren Davidson",
    "Marcy Kaptur",
    "Michael Turner",
    "Marcia Fudge",
    "Pat Tiberi",
    "Tim Ryan",
    "David Joyce",
    "Steve Stivers",
    "Jim Renacci",
    "Jim Bridenstine",
    "Markwayne Mullin",
    "Frank Lucas",
    "Tom Cole",
    "Steve Russell",
    "Suzanne Bonamici",
    "Greg Walden",
    "Earl Blumenauer",
    "Peter DeFazio",
    "Kurt Schrader",
    "Robert Brady",
    "Dwight Evans",
    "Mike Kelly",
    "Scott Perry",
    "Glenn Thompson",
    "Ryan Costello",
    "Pat Meehan",
    "Brian Fitzpatrick",
    "Bill Shuster",
    "Tom Marino",
    "Lou Barletta",
    "Keith Rothfus",
    "Brendan Boyle",
    "Mike Doyle",
    "Charles Dent",
    "Lloyd Smucker",
    "Matthew Cartwright",
    "Tim Murphy",
    "David Cicilline",
    "Jim Langevin",
    "Mark Sanford",
    "Joe Wilson",
    "Jeff Duncan",
    "Trey Gowdy",
    "Ralph Norman",
    "James Clyburn",
    "Tom Rice",
    "Kristi Noem",
    "Phil Roe",
    "John Duncan",
    "Chuck Fleischmann",
    "Scott DesJarlais",
    "Jim Cooper",
    "Diane Black",
    "Marsha Blackburn",
    "David Kustoff",
    "Steve Cohen",
    "Louie Gohmert",
    "Ted Poe",
    "Sam Johnson",
    "John Ratcliffe",
    "Jeb Hensarling",
    "Joe Barton",
    "John Culberson",
    "Kevin Brady",
    "Al Green",
    "Michael McCaul",
    "Michael Conaway",
    "Kay Granger",
    "Mac Thornberry",
    "Randy Weber",
    "Vicente González",
    "Beto O'Rourke",
    "Bill Flores",
    "Shelia Jackson Lee",
    "Jodey Arrington",
    "Joaquin Castro",
    "Lamar Smith",
    "Pete Olson",
    "Will Hurd",
    "Kenny Marchant",
    "Roger Williams",
    "Michael Burgess",
    "Blake Farenthold",
    "Henry Cuellar",
    "Gene Green",
    "Eddie Bernice Johnson",
    "John Carter",
    "Pete Sessions",
    "Marc Veasey",
    "Filemon Vela",
    "Lloyd Doggett",
    "Brian Babin",
    "Rob Bishop",
    "Chris Stewart",
    "Jason Chaffetz",
    "Mia Love",
    "Robert Wittman",
    "Scott Taylor",
    "Robert Scott",
    "Donald McEachin",
    "Tom Garrett",
    "Bob Goodlatte",
    "Dave Brat",
    "Don Beyer",
    "Morgan Griffith",
    "Barbara Comstock",
    "Gerald Connolly",
    "Peter Welch",
    "Suzan DelBene",
    "Rick Larsen",
    "Jaime Herrera Beutler",
    "Dan Newhouse",
    "Cathy McMorris Rodgers",
    "Derek Kilmer",
    "Pramila Jayapal",
    "Dave Reichert",
    "Adam Smith",
    "Denny Heck",
    "Paul Ryan",
    "Mark Pocan",
    "Ron Kind",
    "Gwen Moore",
    "James Sensenbrenner",
    "Glenn Grothman",
    "Sean Duffy",
    "Mike Gallagher",
    "David McKinley",
    "Alex Mooney",
    "Evan Jenkins",
    "Liz Cheney",
];

mod redox {
    use serde_derive::{Deserialize, Serialize};

    /// Data model
    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    enum DataModel {
        PatientAdmin,
    }

    /// Type of event
    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    enum EventType {
        Arrival,
        New,
        Transfer,
        Update,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct Meta {
        /// Data model
        data_model: DataModel,
        /// Type of event
        event_type: EventType,
    }

    /// Patient class is used in many EHRs to determine where to put the patient.
    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    enum PatientClass {
        Inpatient,
        Outpatient,
        Emergency,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct Patient {
        identifiers: Vec<PatientIdentifier>,
    }

    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub enum IdType {
        EPI,
        MRN,
    }

    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct PatientIdentifier {
        /// ID for the patient
        id: String,
        /// Type of ID.
        id_type: IdType,
    }

    impl PatientIdentifier {
        pub fn from_mrn(id: usize) -> PatientIdentifier {
            PatientIdentifier {
                id: format!("{id:>0width$}", id = id, width = 8),
                id_type: IdType::MRN,
            }
        }
    }

    /// An Arrival message is generated when a patient shows up for their visit or when a patient is admitted to the hospital.
    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct Arrival {
        meta: Meta,
        patient: Patient,
        visit: Visit,
    }

    /// A Discharge message is generated when a patient is discharged or checked out from a clinical stay or visit.
    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct Discharge {
        meta: Meta,
        patient: Patient,
        visit: Visit,
    }

    /// A Transfer message is generated when a patient is tranferred from one unit to another.
    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct Transfer {
        meta: Meta,
        patient: Patient,
        visit: Visit,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct Visit {
        /// Patient class is used in many EHRs to determine where to put the patient.
        patient_class: PatientClass,
    }
}

const PROVIDERS: &[&str] = &[
    "Dr. Gregory House",
    "Dr. Feelgood",
    "Dr. Abraham von Helsing",
];

#[derive(Debug)]
struct Nurse {
    name: String,
}

#[derive(Debug, Eq, PartialEq)]
struct Order {
    finish_time: DateTime<Utc>,
}

impl Order {
    fn new(t: DateTime<Utc>) -> Order {
        Order {
            finish_time: t,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Patient {
    name: String,
    id: redox::PatientIdentifier,
    incubation_time: Option<DateTime<Utc>>,
    active_order: Option<Order>,
    orders: PendingOrderQueue,
}

#[derive(Debug, Eq, PartialEq)]
enum BedStatus {
    Present(Patient),
    Reserved(redox::PatientIdentifier),
    Vacant,
}

impl BedStatus {
    fn name(&self) -> &str {
        match self {
            BedStatus::Present(p) => &p.name,
            BedStatus::Reserved(_) => unimplemented!(),
            BedStatus::Vacant => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PendingOrder {
    room_type: RoomType,
    duration: Duration,
}

impl PendingOrder {
    fn new(room_type: RoomType, duration: Duration) -> PendingOrder {
        PendingOrder {
            room_type,
            duration
        }
    }

    fn imaging() -> PendingOrder {
        PendingOrder::new(RoomType::Imaging, Duration::hours(OPERATION_TIME_HOURS))
    }

    fn observation() -> PendingOrder {
        PendingOrder::new(RoomType::Entry, Duration::days(OBSERVATION_DAYS))
    }

    fn operation() -> PendingOrder {
        PendingOrder::new(RoomType::Operating, Duration::hours(OPERATION_TIME_HOURS))
    }
}

#[derive(Debug, Default, Eq, PartialEq, Shrinkwrap)]
#[shrinkwrap(mutable)]
struct PendingOrderQueue(VecDeque<PendingOrder>);

impl PendingOrderQueue {
    fn new(arr: &[PendingOrder]) -> PendingOrderQueue {
        PendingOrderQueue(arr.into_iter().cloned().collect())
    }
}

impl PendingOrderQueue {
    fn random() -> PendingOrderQueue {
        match rand::random::<u8>() % 3 {
            0 => PendingOrderQueue::new(&[
                PendingOrder::observation(),
            ]),
            1 => PendingOrderQueue::new(&[
                PendingOrder::observation(),
                PendingOrder::imaging(),
                PendingOrder::observation(),
            ]),
            2 => PendingOrderQueue::new(&[
                PendingOrder::observation(),
                PendingOrder::operation(),
                PendingOrder::observation(),
            ]),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct Bed {
    incubation_time: Option<DateTime<Utc>>,
    letter: char,
    patient: BedStatus,
    nurse: Rc<Nurse>,
}

impl Bed {
    fn new(letter: char, nurse: Rc<Nurse>) -> Bed {
        Bed {
            incubation_time: None,
            letter,
            patient: BedStatus::Vacant,
            nurse,
        }
    }
    fn update(&mut self, room: &Room, hospital: &Hospital) {
        if let BedStatus::Present(ref mut patient) = self.patient {
            if patient.incubation_time.is_some() && self.incubation_time.is_none() {
                self.incubation_time = Some(hospital.time + Duration::hours(INCUBATION_DURATION_HOURS));
                println!("{}: !!! Patient {} infected {}{}", hospital.time, patient.name, room.number, self.letter);
            }

            if self.incubation_time.is_some() && patient.incubation_time.is_none() {
                patient.incubation_time = Some(hospital.time + Duration::hours(INCUBATION_DURATION_HOURS));
                println!("{}: !!! Patient {} was infected by {}{}", hospital.time, patient.name, room.number, self.letter);
            }
        }
    }
}

#[derive(Debug)]
struct Room {
    beds: [RefCell<Bed>; 2],
    room_type: RoomType,
    number: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RoomType {
    Entry,
    Imaging,
    Operating,
}

const INCUBATION_DURATION_HOURS: i64 = 24;
const BEDS_PER_ROOM: usize = 2;
const NURSE_COUNT: usize = ROOM_COUNT;
const OBSERVATION_DAYS: i64 = 4;
const OPERATION_TIME_HOURS: i64 = 1;
const PATIENT_COUNT: usize = 10000;
const ROOM_COUNT: usize = 300;
const ROOM_START: usize = 100;
const IMAGING_START: usize = 200;
const OPERATING_START: usize = 300;

struct Hospital {
    patient_count: RefCell<usize>,
    rooms: Vec<Room>,
    time: DateTime<Utc>,
}

impl Hospital {
    fn new() -> Hospital {
        let mut nurses: Vec<Rc<Nurse>> = {
            let mut nurse_name_generator =
                names::Generator::new(names::ADJECTIVES, NURSE_NAMES, names::Name::default());
            (0..NURSE_COUNT)
                .map(|id| {
                    Rc::new(Nurse {
                        name: nurse_name_generator.next().unwrap().to_title_case(),
                    })
                }).collect()
        };
        let rooms: Vec<Room> = (ROOM_START..ROOM_START + ROOM_COUNT)
            .map(|room_number| Room {
                beds: if room_number == ROOM_START {
                    [
                        RefCell::new(Bed::new('A', nurses.first().unwrap().clone())),
                        RefCell::new(Bed::new('B', nurses.last().unwrap().clone())),
                    ]
                } else if room_number == (ROOM_START + ROOM_COUNT - 1) {
                    [
                        RefCell::new(Bed::new('A', nurses.pop().unwrap())),
                        RefCell::new(Bed::new('B', nurses.pop().unwrap())),
                    ]
                } else {
                    [
                        RefCell::new(Bed::new('A', nurses.pop().unwrap())),
                        RefCell::new(Bed::new('B', nurses.last().unwrap().clone())),
                    ]
                },
                room_type: if room_number < IMAGING_START {
                    RoomType::Entry
                } else if room_number < OPERATING_START {
                    RoomType::Imaging
                } else {
                    RoomType::Operating
                },
                number: room_number,
            }).collect();
        Hospital {
            patient_count: RefCell::new(0),
            rooms,
            time: chrono::offset::Utc::now(),
        }
    }

    fn admit_patient(&self, mut patient: Patient, room: &Room, bed: &mut Bed) {
        patient.incubation_time = if (rand::random::<u64>() % 50) == 1 { Some(self.time + Duration::hours(INCUBATION_DURATION_HOURS)) } else { None };
        patient.orders = PendingOrderQueue::random();
        patient.active_order = patient.orders.pop_front().map(|x|Order::new(self.time + x.duration));
        println!("{}: Admitted {} to {}{}({:?})", self.time, patient.name, room.number, bed.letter, room.room_type);
        bed.patient = BedStatus::Present(patient);
        bed.update(room, self);
        debug!("Admitted: {:#?}", bed.patient);
        *self.patient_count.borrow_mut() += 1;
    }

    fn discharge_patient(&self, room: &Room, bed: &mut Bed) {
        let mut patient = BedStatus::Vacant;
        std::mem::swap(&mut bed.patient, &mut patient);
        bed.update(room, self);
        debug!("Discharged: {:#?}", patient);
        println!("{}: Discharged {} from {}{}({:?})", self.time, patient.name(), room.number, bed.letter, room.room_type);
        *self.patient_count.borrow_mut() -= 1;
    }

    fn transfer_patient(&self, source_room: &Room, source_bed: &mut Bed, target_room: &Room, target_bed: &mut Bed) {
        std::mem::swap(&mut source_bed.patient, &mut target_bed.patient);
        target_bed.update(target_room, self);
        debug!("Transferred: {:#?}", target_bed.patient);
        println!("{}: Transferred {} from {}{} to {}{}({:?})", self.time, target_bed.patient.name(), source_room.number, source_bed.letter, target_room.number, target_bed.letter, target_room.room_type);
    }

    fn find_bed(&self, room_type: RoomType) -> Option<(&Room, &RefCell<Bed>)> {
        self.rooms
            .iter()
            .filter_map(|room| {
                if room.room_type == room_type {
                    room.beds
                        .iter()
                        .find(|bed| bed.borrow_mut().patient == BedStatus::Vacant)
                        .map(|bed| (room, bed))
                } else {
                    None
                }
            }).next()
    }

    fn find_waiting_bed(&self, patient_id: redox::PatientIdentifier) -> Option<&RefCell<Bed>> {
        self.rooms
            .iter()
            .filter_map(|room| {
                room.beds.iter().find(|bed| {
                    match bed.borrow().patient {
                        BedStatus::Reserved(ref id) => *id == patient_id,
                        BedStatus::Present(_) => false,
                        BedStatus::Vacant => false,
                    }
                })
            }).next()
    }

    fn treat_patients(&mut self, mut patients: Vec<Patient>) {
        while !patients.is_empty() || *self.patient_count.borrow() > 0 {
            for room in self.rooms.iter().rev() {
                for bed in room.beds.iter() {
                    if bed.borrow().patient == BedStatus::Vacant && room.room_type == RoomType::Entry {
                        if let Some(patient) = patients.pop() {
                            self.admit_patient(patient, room, &mut bed.borrow_mut());
                        }
                    } else {
                        let mut target_room_type = None;
                        let mut discharge = false;
                        if let BedStatus::Present(ref patient) =
                            bed.borrow().patient
                        {
                            match patient.active_order {
                                Some(ref order) if order.finish_time <= self.time => {
                                    match patient.orders.front() {
                                        Some(next_order) => {
                                            target_room_type = Some(next_order.room_type);
                                        },
                                        None => {
                                            discharge = true;
                                        },
                                    }
                                },
                                Some(_) => {},
                                None => unimplemented!(),
                            }
                        }
                        if let Some(target_room_type) = target_room_type {
                            if let Some((target_room, target_bed)) = self.find_bed(target_room_type) {
                                if let BedStatus::Present(ref mut patient) = bed.borrow_mut().patient {
                                    patient.orders.pop_front();
                                }
                                self.transfer_patient(
                                    room,
                                    &mut bed.borrow_mut(),
                                    target_room,
                                    &mut target_bed.borrow_mut(),
                                )
                            }
                        } else if discharge {
                            self.discharge_patient(room, &mut bed.borrow_mut())
                        }
                    }
                }
            }
            self.time = self.time + Duration::hours(1);
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let mut patients: Vec<Patient> = {
        let mut patient_name_generator =
            names::Generator::new(names::ADJECTIVES, PATIENT_NAMES, names::Name::default());

        (0..PATIENT_COUNT)
            .map(|id| Patient {
                active_order: None,
                id: redox::PatientIdentifier::from_mrn(id),
                incubation_time: None,
                orders: PendingOrderQueue::default(),
                name: patient_name_generator.next().unwrap().to_title_case(),
            }).collect()
    };

    let mut hospital = Hospital::new();
    hospital.treat_patients(patients);

    Ok(())
}
