import "./styles.css";

type VotolRawFrame = number[];

type VotolSerialFrameProps = {
  frame: VotolRawFrame,
}

function byteToHex(b: number) {
  return b.toString(16).toUpperCase();
}

function byteToBinary(b: number) {
  return b.toString(2).toUpperCase();
}

function fixedPointBytesToNum(byte0: number, byte1: number) {
  return byte0 * 255 + byte1;
}

function flagByteToGear(byte: number) {
  switch (byte & 0b11) {
    case 0b00: return "L";
    case 0b01: return "M";
    case 0b10: return "H";
    case 0b11: return "S";
  }
}

export function VotolFlagsRegister20(props: { flagByte: number}) {
  const b = props.flagByte;
  return (
    <table>
      <tbody>
        <tr>
          <td>Gear</td>
          <td>{flagByteToGear(b)}</td>
        </tr>
        <tr>
          <td>Direction</td>
          <td>{b & 0b100 ? "Reverse" : "Forward"}</td>
        </tr>
        <tr>
          <td>Park</td>
          <td>{b & 0b1000 ? "On" : "Off"}</td>
        </tr>
        <tr>
          <td>Brake</td>
          <td>{b & 0b10000 ? "On" : "Off"}</td>
        </tr>
        <tr>
          <td>Antitheft</td>
          <td>{b & 0b100000 ? "On" : "Off"}</td>
        </tr>
        <tr>
          <td>Side stand</td>
          <td>{b & 0b1000000 ? "On" : "Off"}</td>
        </tr>
        <tr>
          <td>Regen</td>
          <td>{b & 0b10000000 ? "On" : "Off"}</td>
        </tr>
      </tbody>
    </table>
  )
}

function statusRegister21ToString(b21: number) {
  switch (b21) {
    case 0: return 'IDLE';
    case 1: return 'INIT';
    case 2: return 'START';
    case 3: return 'RUN';
    case 4: return 'STOP';
    case 5: return 'BRAKE';
    case 6: return 'WAIT';
    case 7: return 'FAULT';
  }
}

export function VotolSerialFrame(props: VotolSerialFrameProps) {
  const f = props.frame;
  return (
    <ul>
      <li>Controller header - should always be C014: {byteToHex(f[0])}{byteToHex(f[1])}</li>
      <li>Bat voltage: {fixedPointBytesToNum(f[5], f[6])}</li>
      <li>Bat current: {fixedPointBytesToNum(f[7], f[8])}</li>
      <li>Faults: {byteToBinary(f[10])} {byteToBinary(f[11])} {byteToBinary(f[12])} {byteToBinary(f[13])} </li>

      <li>RPM: {fixedPointBytesToNum(f[14], f[15])}</li>

      <li>Controller temp: {f[16] - 50}</li>
      <li>Outside temp: {f[17] - 50}</li>
      <li>Flags:
        <VotolFlagsRegister20 flagByte={f[20]} />
      </li>
      <li>Status: {statusRegister21ToString(f[21])}</li>
    </ul>
  );
}


const EXAMPLE_FRAME_STR = "c0 14 0d 59 42 00 00 00 00 00 00 00 00 84 00 00 4b f0 00 00 01 07 fb 0d";
const EXAMPLE_FRAME = EXAMPLE_FRAME_STR.split(' ').map(b => parseInt(b, 16));

export default function App() {
  return (
    <div className="App">
      <VotolSerialFrame frame={EXAMPLE_FRAME} />
    </div>
  );
}
