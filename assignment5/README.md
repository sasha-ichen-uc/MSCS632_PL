# MSCS632 Assignment 5: Ride Sharing System

## Overview

A class-based Ride Sharing System implemented in **C++** and **Smalltalk**, demonstrating three OOP principles:

- **Encapsulation** — Private member variables with public accessor methods
- **Inheritance** — `StandardRide` and `PremiumRide` extend the base `Ride` class
- **Polymorphism** — Overridden `fare()` and `rideDetails()` methods called on a mixed collection of ride types

## How to Run

### C++
```bash
cd cpp
g++ -std=c++17 -o ride_sharing ride_sharing.cpp
./ride_sharing
```

### Smalltalk (GNU Smalltalk)
```bash
cd smalltalk
gst ride_sharing.st
```

## Classes

| Class | Description |
|-------|-------------|
| `Ride` | Base class with rideID, pickup/dropoff locations, distance, fare() |
| `StandardRide` | Inherits Ride, fare = $1.50/mile |
| `PremiumRide` | Inherits Ride, fare = $3.00/mile |
| `Driver` | Tracks assigned rides (private), calculates total earnings |
| `Rider` | Tracks requested rides, displays ride history and total spent |
