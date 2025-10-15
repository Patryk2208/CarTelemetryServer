import {
    Balance,
    BrakingSignal,
    defaultBalance,
    defaultGG,
    defaultGrip,
    defaultSmoothness,
    GG,
    Grip,
    Smoothness
} from './telemteries.ts';

export interface SharedState {
    gg: GG,
    balance: Balance,
    grip: Grip,
    brakingSignal: BrakingSignal,
    smoothness: Smoothness
}

export function ParseGG(value: string): GG {
    let json = JSON.parse(value);
    return {
        gForceLong: json.g_force_long,
        gForceLat: json.g_force_lat,
        speed: json.speed,
        timestamp: json.timestamp
    } as GG;
}

export function ParseBrake(value: string): BrakingSignal {
    let json = JSON.parse(value);
    return {
        gForce: json.g_force,
        peakBrakeForce: json.peak_brake_force,
        totalBrakingTime: json.total_braking_time,
        timestamp: json.timestamp,
    } as BrakingSignal;
}

export function ParseGrip(value: string): Grip {
    let json = JSON.parse(value);
    return {
        gripForce: json.grip_force,
        maxGripPerCorner: json.max_grip_per_corner,
        maxGripPerRide: json.max_grip_per_ride,
        timestamp: json.timestamp,
    } as Grip;
}

export function ParseBalance(value: string): Balance {
    let json = JSON.parse(value);
    return {
        balanceIndex: json.balance_index,
        timestamp: json.timestamp,
    } as Balance;
}

export function ParseMetrics(value: string): Smoothness {
    let json = JSON.parse(value);
    return {
        smoothnessIndex: json.smoothness_index,
        timestamp: json.timestamp,
    } as Smoothness;
}

export function parseWebSocketMessage(rawJson: string): SharedState | null {
    try {
        const message = JSON.parse(rawJson);

        if (message.type !== 'Telemetry') {
            return null;
        }

        const { data } = message;

        //let parsed = ParseBalance(JSON.stringify(data.balance));
        //console.error(`parsed: ${parsed.balanceIndex} time: ${parsed.timestamp}`);

        return {
            balance: defaultBalance, //ParseBalance(JSON.stringify(data.balance)),
            brakingSignal: ParseBrake(JSON.stringify(data.brakingSignal)),
            gg: defaultGG, //ParseGG(JSON.stringify(data.gg)),
            grip: defaultGrip, //ParseGrip(JSON.stringify(data.grip)),
            smoothness: defaultSmoothness, //ParseMetrics(JSON.stringify(data.smoothness))
        };
    } catch (error) {
        console.error('Failed to parse WebSocket message:', error);
        return null;
    }
}