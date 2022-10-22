import { CPUModule } from "./Modules";

export class Flags extends CPUModule<{}, {}>{
    name = "FLAGS"
    module() {
        return (
            <div className="flags">
                {this.led(this.name)}
                <div className="flags-label">C H O Z</div>
            </div>
        )
    }
}
