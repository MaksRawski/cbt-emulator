import { CPUModule } from "./Modules";

export class LCD extends CPUModule<{},{}>{
    name = "LCD"
    constructor(props: any){
        super(props);
        this.state = new Uint8Array(80);
    }
    module(){
        return <div className="LCD">
            <div className="LCD-row"></div>
            <div className="LCD-row"></div>
        </div>
    }
}
