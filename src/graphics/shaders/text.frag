#version 330 core

in VS_OUTPUT {
    vec3 Color;
    vec2 TexCoord;
} IN;

out vec4 Color;

uniform sampler2D ourTexture;

void main()
{
    vec4 texel = texture(ourTexture, IN.TexCoord);
    if(texel.a == 0.0)
        discard;
    Color = texel;
}