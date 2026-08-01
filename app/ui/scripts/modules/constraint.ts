type FixX = {
    "FixX": {
        point: number,
        position: number
    }
};
type FixY = { 
    "FixY": {
        point: number,
        position: number
    }
};
type FixZ = { 
    "FixZ": {
        point: number,
        position: number
    }
};
type Distance = { 
    "Distance": {
        point_1: number,
        point_2: number,
        distance: number
    }
};
type Angle = { 
    "Angle": {
        point_1: number,
        point_2: number,
        point_3: number,
        angle: number
    }
};
export type Constraint = FixX | FixY | FixZ | Distance | Angle;
