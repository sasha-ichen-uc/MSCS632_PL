#include <iostream>
#include <string>
#include <vector>
#include <memory>

using namespace std;

// ==================== Base Ride Class ====================
// Demonstrates ENCAPSULATION: private data with public accessors
// Demonstrates INHERITANCE: base class for StandardRide and PremiumRide

class Ride {
private:
    string rideID;
    string pickupLocation;
    string dropoffLocation;
    double distance;

public:
    Ride(string id, string pickup, string dropoff, double dist)
        : rideID(id), pickupLocation(pickup), dropoffLocation(dropoff), distance(dist) {}

    virtual ~Ride() = default;

    string getRideID() const { return rideID; }
    string getPickup() const { return pickupLocation; }
    string getDropoff() const { return dropoffLocation; }
    double getDistance() const { return distance; }

    // Virtual method for POLYMORPHISM — overridden by subclasses
    virtual double fare() const {
        return distance * 1.50;
    }

    virtual string rideType() const {
        return "Basic";
    }

    virtual void rideDetails() const {
        cout << "  [" << rideType() << "] Ride " << rideID
             << ": " << pickupLocation << " -> " << dropoffLocation
             << " | " << distance << " mi | $" << fare() << endl;
    }
};

// ==================== StandardRide Subclass ====================

class StandardRide : public Ride {
public:
    StandardRide(string id, string pickup, string dropoff, double dist)
        : Ride(id, pickup, dropoff, dist) {}

    double fare() const override {
        return getDistance() * 1.50;  // $1.50 per mile
    }

    string rideType() const override {
        return "Standard";
    }
};

// ==================== PremiumRide Subclass ====================

class PremiumRide : public Ride {
public:
    PremiumRide(string id, string pickup, string dropoff, double dist)
        : Ride(id, pickup, dropoff, dist) {}

    double fare() const override {
        return getDistance() * 3.00;  // $3.00 per mile
    }

    string rideType() const override {
        return "Premium";
    }
};

// ==================== Driver Class ====================
// Demonstrates ENCAPSULATION: assignedRides is private

class Driver {
private:
    string driverID;
    string name;
    double rating;
    vector<shared_ptr<Ride>> assignedRides;  // private, accessed via methods

public:
    Driver(string id, string n, double r)
        : driverID(id), name(n), rating(r) {}

    void addRide(shared_ptr<Ride> ride) {
        assignedRides.push_back(ride);
    }

    void getDriverInfo() const {
        cout << "Driver: " << name << " (ID: " << driverID
             << ", Rating: " << rating << ")" << endl;
        cout << "  Completed rides: " << assignedRides.size() << endl;
        for (const auto& ride : assignedRides) {
            ride->rideDetails();
        }
    }

    double totalEarnings() const {
        double total = 0;
        for (const auto& ride : assignedRides) {
            total += ride->fare();
        }
        return total;
    }
};

// ==================== Rider Class ====================

class Rider {
private:
    string riderID;
    string name;
    vector<shared_ptr<Ride>> requestedRides;

public:
    Rider(string id, string n) : riderID(id), name(n) {}

    void requestRide(shared_ptr<Ride> ride) {
        requestedRides.push_back(ride);
    }

    void viewRides() const {
        cout << "Rider: " << name << " (ID: " << riderID << ")" << endl;
        cout << "  Ride history:" << endl;
        double total = 0;
        for (const auto& ride : requestedRides) {
            ride->rideDetails();
            total += ride->fare();
        }
        cout << "  Total spent: $" << total << endl;
    }
};

// ==================== Main: System Functionality ====================

int main() {
    cout << "========================================" << endl;
    cout << "  Ride Sharing System (C++)" << endl;
    cout << "========================================" << endl;

    // Create rides of different types
    auto r1 = make_shared<StandardRide>("R001", "Downtown", "Airport", 12.5);
    auto r2 = make_shared<PremiumRide>("R002", "Hotel", "Convention Center", 5.0);
    auto r3 = make_shared<StandardRide>("R003", "Mall", "University", 8.0);
    auto r4 = make_shared<PremiumRide>("R004", "Airport", "Hotel", 14.0);

    // Demonstrate POLYMORPHISM: different ride types in one collection
    cout << "\n--- All Rides (Polymorphic Dispatch) ---" << endl;
    vector<shared_ptr<Ride>> allRides = {r1, r2, r3, r4};
    for (const auto& ride : allRides) {
        ride->rideDetails();  // calls the correct overridden method
    }

    // Create driver and assign rides
    cout << "\n--- Driver Info ---" << endl;
    Driver driver1("D001", "Alice", 4.8);
    driver1.addRide(r1);
    driver1.addRide(r2);
    driver1.addRide(r4);
    driver1.getDriverInfo();
    cout << "  Total earnings: $" << driver1.totalEarnings() << endl;

    // Create rider and request rides
    cout << "\n--- Rider Info ---" << endl;
    Rider rider1("U001", "Bob");
    rider1.requestRide(r1);
    rider1.requestRide(r3);
    rider1.viewRides();

    return 0;
}
