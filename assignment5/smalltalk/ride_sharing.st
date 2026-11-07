"Ride Sharing System in Smalltalk (GNU Smalltalk)"
"Demonstrates encapsulation, inheritance, and polymorphism"

"==================== Ride Base Class ===================="
"ENCAPSULATION: instance variables accessed only through methods"
"INHERITANCE: StandardRide and PremiumRide inherit from Ride"

Object subclass: #Ride
    instanceVariableNames: 'rideID pickupLocation dropoffLocation distance'
    classVariableNames: ''
    poolDictionaries: ''
    category: 'RideSharing'.

Ride >> initWithID: anID pickup: aPickup dropoff: aDropoff distance: aDist [
    rideID := anID.
    pickupLocation := aPickup.
    dropoffLocation := aDropoff.
    distance := aDist.
]

Ride >> rideID [ ^rideID ]
Ride >> pickupLocation [ ^pickupLocation ]
Ride >> dropoffLocation [ ^dropoffLocation ]
Ride >> distance [ ^distance ]

"Virtual method — overridden by subclasses for POLYMORPHISM"
Ride >> fare [
    ^distance * 1.50
]

Ride >> rideType [
    ^'Basic'
]

Ride >> rideDetails [
    Transcript show: '  [', self rideType, '] Ride ', rideID,
        ': ', pickupLocation, ' -> ', dropoffLocation,
        ' | ', distance printString, ' mi | $',
        self fare printString; cr.
]

"==================== StandardRide Subclass ===================="

Ride subclass: #StandardRide
    instanceVariableNames: ''
    classVariableNames: ''
    poolDictionaries: ''
    category: 'RideSharing'.

StandardRide >> fare [
    ^distance * 1.50
]

StandardRide >> rideType [
    ^'Standard'
]

"==================== PremiumRide Subclass ===================="

Ride subclass: #PremiumRide
    instanceVariableNames: ''
    classVariableNames: ''
    poolDictionaries: ''
    category: 'RideSharing'.

PremiumRide >> fare [
    ^distance * 3.00
]

PremiumRide >> rideType [
    ^'Premium'
]

"==================== Driver Class ===================="
"ENCAPSULATION: assignedRides is private, accessed through methods"

Object subclass: #Driver
    instanceVariableNames: 'driverID name rating assignedRides'
    classVariableNames: ''
    poolDictionaries: ''
    category: 'RideSharing'.

Driver >> initWithID: anID name: aName rating: aRating [
    driverID := anID.
    name := aName.
    rating := aRating.
    assignedRides := OrderedCollection new.
]

Driver >> addRide: aRide [
    assignedRides add: aRide.
]

Driver >> totalEarnings [
    | total |
    total := 0.
    assignedRides do: [:ride | total := total + ride fare].
    ^total
]

Driver >> getDriverInfo [
    Transcript show: 'Driver: ', name, ' (ID: ', driverID,
        ', Rating: ', rating printString, ')'; cr.
    Transcript show: '  Completed rides: ',
        assignedRides size printString; cr.
    assignedRides do: [:ride | ride rideDetails].
]

"==================== Rider Class ===================="

Object subclass: #Rider
    instanceVariableNames: 'riderID name requestedRides'
    classVariableNames: ''
    poolDictionaries: ''
    category: 'RideSharing'.

Rider >> initWithID: anID name: aName [
    riderID := anID.
    name := aName.
    requestedRides := OrderedCollection new.
]

Rider >> requestRide: aRide [
    requestedRides add: aRide.
]

Rider >> viewRides [
    | total |
    total := 0.
    Transcript show: 'Rider: ', name, ' (ID: ', riderID, ')'; cr.
    Transcript show: '  Ride history:'; cr.
    requestedRides do: [:ride |
        ride rideDetails.
        total := total + ride fare].
    Transcript show: '  Total spent: $', total printString; cr.
]

"==================== Main Demo ===================="

| r1 r2 r3 r4 allRides driver1 rider1 |

Transcript show: '========================================'; cr.
Transcript show: '  Ride Sharing System (Smalltalk)'; cr.
Transcript show: '========================================'; cr.

"Create rides of different types"
r1 := StandardRide new initWithID: 'R001' pickup: 'Downtown' dropoff: 'Airport' distance: 12.5.
r2 := PremiumRide new initWithID: 'R002' pickup: 'Hotel' dropoff: 'Convention Center' distance: 5.0.
r3 := StandardRide new initWithID: 'R003' pickup: 'Mall' dropoff: 'University' distance: 8.0.
r4 := PremiumRide new initWithID: 'R004' pickup: 'Airport' dropoff: 'Hotel' distance: 14.0.

"POLYMORPHISM: different ride types in one collection"
Transcript cr; show: '--- All Rides (Polymorphic Dispatch) ---'; cr.
allRides := OrderedCollection withAll: { r1. r2. r3. r4 }.
allRides do: [:ride | ride rideDetails].

"Create driver and assign rides"
Transcript cr; show: '--- Driver Info ---'; cr.
driver1 := Driver new initWithID: 'D001' name: 'Alice' rating: 4.8.
driver1 addRide: r1.
driver1 addRide: r2.
driver1 addRide: r4.
driver1 getDriverInfo.
Transcript show: '  Total earnings: $', driver1 totalEarnings printString; cr.

"Create rider and request rides"
Transcript cr; show: '--- Rider Info ---'; cr.
rider1 := Rider new initWithID: 'U001' name: 'Bob'.
rider1 requestRide: r1.
rider1 requestRide: r3.
rider1 viewRides.
