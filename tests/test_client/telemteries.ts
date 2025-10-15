//todo better timestamp representation

export interface GG {
    gForceLong: number,
    gForceLat: number,
    speed: number,
    timestamp: number,
}

export const defaultGG = {
    gForceLong: 0,
    gForceLat: 0,
    speed: 0,
    timestamp: 0,
}

export interface BrakingSignal {
    gForce: number,
    totalBrakingTime: number | null,
    peakBrakeForce: number | null,
    timestamp: number,
}

export const defaultBrakingSignal = {
    gForce: 0,
    totalBrakingTime: null,
    peakBrakeForce: null,
    timestamp: 0,
}

export interface Grip {
    gripForce: number
    maxGripPerCorner: number | null,
    maxGripPerRide: number | null,
    timestamp: number,
}

export const defaultGrip = {
    gripForce: 0,
    maxGripPerCorner: null,
    maxGripPerRide: null,
    timestamp: 0,
}

export interface Balance {
    balanceIndex: number,
    timestamp: number,
}

export const defaultBalance = {
    balanceIndex: 0,
    timestamp: 0,
}

export interface Smoothness {
    smoothnessIndex: number,
    timestamp: number,
}

export const defaultSmoothness = {
    smoothnessIndex: 0,
    timestamp: 0,
}