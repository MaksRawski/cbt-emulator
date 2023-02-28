import React from 'react';
import { cpu, reset_cpu } from './cpu';
import Slider from '@mui/material/Slider';
import SpeedIcon from '@mui/icons-material/Speed';
import { Button, IconButton } from '@mui/material';
import { Pause, PlayArrow } from '@mui/icons-material';

let off = '○';
let on = '●';

interface CLKstate {
    speed: number,
    auto: boolean,
    hlt: boolean,
    v: boolean,
}

export class CLK extends React.Component<{}, CLKstate> {
    private autoID?: NodeJS.Timer;

    constructor(props: any) {
        super(props);

        // - value: current clock edge
        // - speed: clock speed in hz,
        // - auto: determines whether the clock should tick on its own
        this.state = { auto: false, speed: 1, hlt: false, v: false };

        // react by default doesn't bind methods to itself (why????)
        this.updateSpeed = this.updateSpeed.bind(this);
        this.tick = this.tick.bind(this);
        this.auto = this.auto.bind(this);

    }
    componentDidMount(): void {
        global.halt = () => {
            console.log("halting");
            this.setState({ hlt: true });
        };
    }

    tick() {
        if (this.state.v || this.state.hlt) return
        cpu.tick();
        this.setState({ v: true });

        setTimeout(() => {
            this.setState({ v: false });
        }, 1000 / this.state.speed);
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
    updateSpeed(_ev: Event, value: number | Array<number>, _activeThumb: number) {
        value = value as number;

        this.setState({ speed: this.speedScale(value) })
        clearInterval(this.autoID);
        if (this.state.auto) {
            this.autoID = setInterval(this.tick, 1000 / this.state.speed);
        }
    }
    /** logarithmic slider: https://stackoverflow.com/a/846249 */
    speedScale(v: number) {
        return Math.round(Math.exp(v * Math.log(1000) / 1000));
    }
    render() {
        return (
            <div className="clock">
                <div className="clockMode">
                    <Button variant="contained"
                        disabled={this.state.auto || this.state.hlt}
                        onClick={this.tick}
                    >PULSE</Button>
                    <IconButton disabled={this.state.hlt} sx={{ borderRadius: "5px" }}
                                onClick={this.state.hlt ? reset_cpu : this.auto}>
                        <PlayArrow sx={{ fontSize: "1.8em", display: this.state.auto ? "none" : "block" }} />
                        <Pause sx={{ fontSize: "1.8em",
                                     display: !this.state.auto ? "none" : "block",
                                     color: this.state.hlt ? "gray" : "white" }} />

                    </IconButton>

                </div>
                <div className="slider">
                    <SpeedIcon />
                    <Slider
                        min={1}
                        step={1}
                        max={1000}
                        defaultValue={4}
                        scale={this.speedScale}
                        valueLabelDisplay="auto"
                        valueLabelFormat={v => `${v} Hz`}
                        arial-label="Speed"
                        aria-valuetext="Hz"
                        onChange={this.updateSpeed} />
                    <p className="LED">{this.state.v ? on : off}</p>
                </div>
            </div >
        );
    }
}
