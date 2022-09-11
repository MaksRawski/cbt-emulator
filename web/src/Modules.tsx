import React from 'react';
export { CLK };

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
    value: boolean,
    speed: number,
    auto: boolean
}

class CLK extends React.Component<{}, CLKstate> {
    constructor(props: any) {
        super(props);

        // speed is in hz,
        // auto determines whether the clock should update itself
        // based on the speed
        this.state = { value: false, speed: 1, auto: false };

        // react by default doesn't bind methods to itself (why????)
        this.pulse = this.pulse.bind(this);
        this.updateSpeed = this.updateSpeed.bind(this);
    }
    pulse() {
        if (this.state.value === true) return
        this.setState({ value: true });
        setTimeout(() => {
            this.setState({ value: false });
        }, 1000 / this.state.speed);
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
                    <input type="button" disabled value="PULSE" onClick={this.pulse} />
                    <div className="checkbox">
                        <input type="checkbox" name="auto" onClick={() => { this.setState({ auto: !this.state.auto }) }} />
                        <label htmlFor="auto" />
                    </div>
                </div>
                <p className="LED">{this.state.value ? on : off}</p>
                <input type="range" name="speed" min="0" max="1000" onChange={this.updateSpeed} />
                <p className="speedValue">{this.state.speed}</p>
            </div>
        );
    }
}

/* class LED extends React.Component<{ value: boolean }>{
*     render() {
*     }
* } */
