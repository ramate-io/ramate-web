import * as jspb from 'google-protobuf'



export class EchoCom extends jspb.Message {
  getText(): string;
  setText(value: string): EchoCom;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EchoCom.AsObject;
  static toObject(includeInstance: boolean, msg: EchoCom): EchoCom.AsObject;
  static serializeBinaryToWriter(message: EchoCom, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EchoCom;
  static deserializeBinaryFromReader(message: EchoCom, reader: jspb.BinaryReader): EchoCom;
}

export namespace EchoCom {
  export type AsObject = {
    text: string,
  }
}

