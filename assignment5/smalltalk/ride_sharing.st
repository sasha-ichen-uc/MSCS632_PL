"Ride Sharing System in GNU Smalltalk"
"Demonstrates encapsulation, inheritance, and polymorphism"

"==================== Ride Base Class ===================="
"ENCAPSULATION: instance variables accessed only through methods"
"INHERITANCE: StandardRide and PremiumRide inherit from Ride"

Object subclass: Ride [
    | rideID pickupLocation dropoffLocation distance |

    initWithID: anID pickup: aPickup dropoff: aDropoff distance: aDist [
        rideID := anID.
        pickupLocation := aPickup.
        dropoffLocation := aDropoff.
        distance := aDist.
    ]

    rideID [ ^rideID ]
    pickupLocation [ ^pickupLocation ]
    dropoffLocation [ ^dropoffLocation ]
    distance [ ^distance ]

    "Virtual method — overridden by subclasses for POLYMORPHISM"
    fare [
        ^distance * 1.50
    ]

    rideType [
        ^'Basic'
    ]

    rideDetails [
        Transcript show: '  [', self rideType, '] Ride ', rideID,
            ': ', pickupLocation, ' -> ', dropoffLocation,
            ' | ', distance printString, ' mi | $',
            self fare printString; cr.
    ]
]

"==================== StandardRide Subclass ===================="

Ride subclass: StandardRide [
    fare [
        ^self distance * 1.50
    ]

    rideType [
        ^'Standard'
    ]
]

"==================== PremiumRide Subclass ===================="

Ride subclass: PremiumRide [
    fare [
        ^self distance * 3.00
    ]

    rideType [
        ^'Premium'
    ]
]

"==================== Driver Class ===================="
"ENCAPSULATION: assignedRides is private, accessed through methods"

Object subclass: Driver [
    | driverID name rating assignedRides |

    initWithID: anID name: aName rating: aRating [
        driverID := anID.
        name := aName.
        rating := aRating.
        assignedRides := OrderedCollection new.
    ]

    addRide: aRide [
        assignedRides add: aRide.
    ]

    totalEarnings [
        | total |
        total := 0.
        assignedRides do: [:ride | total := total + ride fare].
        ^total
    ]

    getDriverInfo [
        Transcript show: 'Driver: ', name, ' (ID: ', driverID,
            ', Rating: ', rating printString, ')'; cr.
        Transcript show: '  Completed rides: ',
            assignedRides size printString; cr.
        assignedRides do: [:ride | ride rideDetails].
    ]
]

"==================== Rider Class ===================="

Object subclass: Rider [
    | riderID name requestedRides |

    initWithID: anID name: aName [
        riderID := anID.
        name := aName.
        requestedRides := OrderedCollection new.
    ]

    requestRide: aRide [
        requestedRides add: aRide.
    ]

    viewRides [
        | total |
        total := 0.
        Transcript show: 'Rider: ', name, ' (ID: ', riderID, ')'; cr.
        Transcript show: '  Ride history:'; cr.
        requestedRides do: [:ride |
            ride rideDetails.
            total := total + ride fare].
        Transcript show: '  Total spent: $', total printString; cr.
    ]
]

"==================== Main Demo ===================="

| r1 r2 r3 r4 allRides driver1 rider1 |

Transcript show: '========================================'; cr.
Transcript show: '  Ride Sharing System (Smalltalk)'; cr.
Transcript show: '========================================'; cr.

r1 := StandardRide new initWithID: 'R001' pickup: 'Downtown' dropoff: 'Airport' distance: 12.5.
r2 := PremiumRide new initWithID: 'R002' pickup: 'Hotel' dropoff: 'Convention Center' distance: 5.0.
r3 := StandardRide new initWithID: 'R003' pickup: 'Mall' dropoff: 'University' distance: 8.0.
r4 := PremiumRide new initWithID: 'R004' pickup: 'Airport' dropoff: 'Hotel' distance: 14.0.

Transcript cr; show: '--- All Rides (Polymorphic Dispatch) ---'; cr.
allRides := OrderedCollection withAll: { r1. r2. r3. r4 }.
allRides do: [:ride | ride rideDetails].

Transcript cr; show: '--- Driver Info ---'; cr.
driver1 := Driver new initWithID: 'D001' name: 'Alice' rating: 4.8.
driver1 addRide: r1.
driver1 addRide: r2.
driver1 addRide: r4.
driver1 getDriverInfo.
Transcript show: '  Total earnings: $', driver1 totalEarnings printString; cr.

Transcript cr; show: '--- Rider Info ---'; cr.
rider1 := Rider new initWithID: 'U001' name: 'Bob'.
rider1 requestRide: r1.
rider1 requestRide: r3.
rider1 viewRides.
