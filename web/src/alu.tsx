import { ReactNode } from "react";
import { CPUModule } from "./Modules";

const OFF = '○';
const ON = '●';

export type FlagsType = {c: boolean, h: boolean, o: boolean, z: boolean}

export class Flags extends CPUModule<{}, FlagsType>{
    name = "FLAGS"

    constructor(props: any) {
        super(props);
        this.state = {c: false, h: false, o: false, z: false};
    }
    componentDidMount(): void {
        global.set_flags = (v: FlagsType) => {
            this.setState(v);
        }
    }
    module() {
        return (
            <div className="flags">
                <div className="LED">
                    {this.state.c? ON:OFF}
                    {this.state.h? ON:OFF}
                    {this.state.o? ON:OFF}
                    {this.state.z? ON:OFF}
                </div>
                <div className="flags-label">C H O Z</div>
            </div>
        )
    }
}
