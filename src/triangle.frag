#version 330 core

in vec4 gl_FragCoord;
out vec4 Color;

void main()
{
    float zre = 0.0f;
    float zim = 0.0f;
    float cre = 0.4f*(2.0f*(gl_FragCoord.x / 800.0f) - 0.3);
    float cim = 0.4f*(2.0f*(gl_FragCoord.y / 800.0f) - 0.3);

    float tr = 0.0;
    float ti = 0.0;
    float exit = -1.0f;
    for(int i = 0; i < 200; i++) {
        tr = zre;
        ti = zim;
        zre = tr*tr - ti*ti + cre;
        zim = 2.0f*tr*ti + cim;
        if (zre*zre + zim*zim >= 4.0f && exit < 0.0f) {
            exit = i;
        }
    }

    float r = 0.0f;
    float g = 0.0f;
    float b = 0.0f;

    if (exit >= 0) {
        r = exit/199;
    } else {
        float dis = sqrt(zre*zre + zim*zim);
        b = dis/2.0;
    }

    Color = vec4(r, g, b, 1.0f);
}