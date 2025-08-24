export interface Image {
    width: number;
    height: number;
    data: ImageData;
}
export declare function load_from_url(url: RequestInfo): Promise<Image>;
export declare function load_from_arraybuffer(buffer: ArrayBuffer): Promise<Image>;
