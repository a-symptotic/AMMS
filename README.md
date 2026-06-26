# Aircraft Maintenance Management System (AMMS)

## CS50x Final Project

### Author

**Subhadarsan Jena**

---

# Project Overview

Aircraft Maintenance Management System (AMMS) is a command-line application developed in **Rust** using **SQLite** for persistent data storage.

The purpose of this project is to provide a simple maintenance management system for aircraft. It allows engineers or maintenance personnel to maintain aircraft records, store maintenance history, and calculate inspection intervals.

This project was developed as the **Final Project for Harvard's CS50x – Introduction to Computer Science**.

---

# Problem Statement

Aircraft maintenance organizations require proper documentation of aircraft information and maintenance activities.

For small organizations or training environments, a lightweight command-line application can provide an easy way to:

* Maintain aircraft records
* Record maintenance activities
* Store maintenance history
* Calculate inspection due intervals

This project provides a simple solution without requiring complex enterprise software.

---

# Features

## Aircraft Management

* Add Aircraft
* View Aircraft Fleet
* Delete Aircraft

Each aircraft stores:

* Registration Number
* Aircraft Type
* Total Flight Hours

---

## Maintenance Management

Maintenance records include:

* Aircraft
* Maintenance Date
* Engineer Name
* Aircraft Hours
* Work Performed

All records are permanently stored inside an SQLite database.

---

## Inspection Calculator

The inspection calculator determines:

* Current Hours
* Inspection Interval
* Next Inspection Due
* Remaining Hours Until Inspection

---

## Database

SQLite is used for persistent storage.

Tables:

### Aircraft

* ID
* Registration
* Aircraft Type
* Total Hours

### Maintenance Logs

* ID
* Aircraft ID
* Maintenance Date
* Engineer Name
* Aircraft Hours
* Work Performed

The Maintenance table is linked to the Aircraft table using a foreign key relationship.

---

# Technologies Used

* Rust
* SQLite
* rusqlite
* Cargo

---

# Project Structure

```text
amms/
│
├── Cargo.toml
├── Cargo.lock
├── README.md
├── amms.db
│
└── src/
    ├── aircraft.rs
    ├── db.rs
    ├── inspection.rs
    ├── maintenance.rs
    ├── utils.rs
    └── main.rs
```

---

# How to Build

Clone the repository:

```bash
git clone <repository-url>
```

Enter the project directory:

```bash
cd amms
```

Build:

```bash
cargo build
```

Run:

```bash
cargo run
```

---

# Sample Workflow

1. Add an aircraft.
2. View all aircraft.
3. Record maintenance performed.
4. View maintenance history.
5. Calculate the next inspection due.

---

# Future Improvements

Future versions of AMMS may include:

* Aircraft search
* Flight hour updates
* Scheduled inspections
* Component tracking
* Defect reporting
* ATA chapter support
* User authentication
* PDF report generation
* Web-based interface

---

# AI Usage

During the development of this project, AI tools were used to assist with:

* Rust syntax guidance
* SQLite query refinement
* Code debugging
* Project structure recommendations

All final implementation decisions, testing, integration, and customization were performed by the author.

---

# License

This project is developed for educational purposes as part of the CS50x Final Project.
