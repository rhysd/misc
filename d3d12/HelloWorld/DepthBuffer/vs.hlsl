struct VSInput {
    float3 Position : POSITION;
    float4 Color : COLOR;
};

struct VSOutput {
    float4 Position : SV_POSITION;
    float4 Color : COLOR;
};

cbuffer Transform : register(b0) {
    float4x4 World : packoffset(c0);
    float4x4 View : packoffset(c4);
    float4x4 Proj : packoffset(c8);
}

VSOutput main(VSInput input) {
    VSOutput output;

    float4 local = float4(input.Position, 1.0f);
    float4 world = mul(World, local);
    float4 view = mul(View, world);
    float4 proj = mul(Proj, view);

    output.Position = proj;
    output.Color = input.Color;

    return output;
}
