struct VSOutput {
    float4 Position : SV_POSITION;
    float4 Color : COLOR;
};

struct PSOutput {
    float4 Color : SV_TARGET0;
};

PSOutput main(VSOutput input) {
    PSOutput output;

    output.Color = input.Color;

    return output;
}
