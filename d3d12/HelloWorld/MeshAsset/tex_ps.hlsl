struct VSOutput {
    float4 Position : SV_POSITION;
    float2 TexCoord : TEXCOORD;
};

struct PSOutput {
    float4 Color : SV_TARGET0;
};

SamplerState ColorSmp : register(s0);
Texture2D ColorMap : register(t0);

PSOutput main(VSOutput input) {
    PSOutput output;
    output.Color = ColorMap.Sample(ColorSmp, input.TexCoord);
    return output;
}