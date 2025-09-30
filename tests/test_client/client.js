import WebSocket from 'ws';
import { performance } from 'perf_hooks';

class TelemetryTester {
    constructor() {
        this.ws = null;
        this.startTime = 0,
        this.messageCount = 0,
        this.lastTimestamp = 0,
        this.minInterval = Number.MAX_VALUE,
        this.maxInterval = 0,
        this.avgInterval = 0,
        this.minLatency = Number.MAX_VALUE,
        this.maxLatency = 0,
        this.avgLatency = 0
    }

    connect(url) {
        this.ws = new WebSocket(url);

        this.ws.on('open', () => {
            console.log('Connected to telemetry server');
            this.startTime = performance.now();
            this.lastTimestamp = performance.now();
        });

        this.ws.on('message', (data) => {
            this.handleMessage(data);
        });

        this.ws.on('close', () => {
            this.printFinalStats();
        });

        this.ws.on('error', (err) => {
            console.error('❌ WebSocket error:', err.message);
        });
    }

    handleMessage(data) {
        try {
            const message = JSON.parse(data);

            this.messageCount++;
            const now = performance.now();

            if (this.lastTimestamp) {
                const interval = now - this.lastTimestamp;
                this.minInterval = Math.min(this.minInterval, interval);
                this.maxInterval = Math.max(this.maxInterval, interval);
                this.avgInterval = (this.avgInterval * (this.messageCount - 1) + interval) / this.messageCount;
            }
            this.lastTimestamp = now;

            if (message.timestamp) {
                const serverTime = message.timestamp;
                const clientTime = Date.now();
                const latency = clientTime - serverTime;
                this.maxLatency = Math.max(this.maxLatency, latency);
                this.minLatency = Math.min(this.minLatency, latency);
                this.avgLatency = (this.avgLatency * (this.messageCount - 1) + latency) / this.messageCount;
            }

            if (this.messageCount % 200 === 0) {
                this.printCurrentStats();
            }

            if (this.messageCount % 10000 === 1) {
                console.log('Sample message:', {
                    message
                });
            }

        } catch (err) {
            console.error('Failed to parse message:', err.message);
        }
    }

    printCurrentStats() {
        const elapsed = (performance.now() - this.startTime) / 1000;
        const rate = this.messageCount / elapsed;

        console.log(`
           Messages: ${this.messageCount}
           Rate: ${rate.toFixed(2)} Hz
           Avg Interval: ${this.avgInterval.toFixed(2)}ms
           Min/Max Interval: ${this.minInterval.toFixed(2)}/${this.maxInterval.toFixed(2)}ms
           Avg Latency: ${this.avgLatency.toFixed(2)}ms
           Duration: ${elapsed.toFixed(2)}s
            `
        );
    }

    printFinalStats() {
        console.log('\n🎯 FINAL STATISTICS:');
        this.printCurrentStats();
    }
}

const tester = new TelemetryTester();
tester.connect('ws://localhost:8080');

setTimeout(() => {
    console.log('Test completed after 30 seconds');
    process.exit(0);
}, 30000);