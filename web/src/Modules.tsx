import React from 'react';
import { cpu } from './cpu';

let off = '○';
let on = '●';

/* function to_binary(n: number, width: number): string {
*     let res = "";
*     for (let i = 0; i < width; i++) {
*         res += n & (1 << i) ? full : empty;
*     }
*     return res;
* } */

interface CLKstate {
    speed: number,
    auto: boolean
}
interface CLKprops {
    clkState: { v: boolean, d: React.Dispatch<React.SetStateAction<boolean>> }
}

export class CLK extends React.Component<CLKprops, CLKstate> {
    private clkState: CLKprops["clkState"];
    private autoID?: NodeJS.Timer;

    constructor(props: any) {
        super(props);

        // - value: current clock edge
        // - speed: clock speed in hz,
        // - auto: determines whether the clock should tick on its own
        this.state = { auto: false, speed: 1 };
        this.clkState = props.clkState;

        // react by default doesn't bind methods to itself (why????)
        this.updateSpeed = this.updateSpeed.bind(this);
        this.tick = this.tick.bind(this);
        this.auto = this.auto.bind(this);

    }
    tick() {
        if (this.clkState.v === true) return
        cpu.tick();
        console.log("[%d.%d] instruction: %d, bus: %d", cpu.pc.lo(), cpu.clock.utime, cpu.ir[0], cpu.bus[0]);
        this.clkState.d(true);

        setTimeout(() => {
            this.clkState.d(false);
        }, 1000 / this.state.speed);

        if (cpu.pc.lo() === 38) {
            console.log("done!");
            console.log("LCD: ", cpu.lcd.content());
        }
    }
    auto() {
        if (this.state.auto) {
            this.setState({ auto: false });
            clearInterval(this.autoID);
        } else {
            this.setState({ auto: true });
            this.autoID = setInterval(this.tick, 1000 / this.state.speed);
        }
    }
    updateSpeed() {
        // logarithmic slider
        // https://stackoverflow.com/a/846249
        let speedRange = document.querySelector("input[name='speed']") as HTMLInputElement;
        let position = parseInt(speedRange.value);
        let max = parseInt(speedRange.max);
        let scale = Math.log(max) / max;
        let speed = Math.exp(scale * position);

        this.setState({ speed: Math.floor(speed) })
    }
    render() {
        return (
            <div>
                <div className="clockMode">
                    <input type="button" value="PULSE" onClick={this.tick} />
                    <label className="checkbox">
                        <input type="checkbox" name="auto" onClick={this.auto} />
                        <svg className="checkmark" version="1.1" xmlns="http://www.w3.org/2000/svg" viewBox='0 0 40 20'>
                            <g transform="rotate(180,16,16)">
                                <polygon points="8,16 32,0 32,32 " />
                                <rect y="4" width="8" height="24" x="0" />
                            </g>
                        </svg>
                    </label>
                </div>
                <p className="LED">{this.clkState.v ? on : off}</p>
                <input type="range" name="speed" min="0" max="1000" onChange={this.updateSpeed} />
                <p className="speedValue">{this.state.speed}</p>
            </div >
        );
    }
}

/* class LED extends React.Component<{ value: boolean }>{
*     render() {
*     }
* } */
