#version 330

out vec4 colorOut;

uniform float zoom;
uniform float screen_width;
uniform float screen_height;
uniform float center_x;
uniform float center_y;
uniform int itr;

vec4 map_to_color(float t, int itr) {
    float color = t;

    return vec4(color, color, color, 1.0);
}

void main()
{
    float screen_ratio = screen_width / screen_height;
    vec2 screen_size = vec2(screen_width, screen_height);
    vec2 center = vec2(center_x, center_y);

    vec2 z, c;
    c.x = screen_ratio * (gl_FragCoord.x / screen_size.x - 0.5);
    c.y = (gl_FragCoord.y / screen_size.y - 0.5);

    c.x /= zoom;
    c.y /= zoom;

    c.x += center.x;
    c.y += center.y;

    int i;
    int last_itr = 0;
    for(i = 0; i < itr; i++) {
        float x = (z.x * z.x - z.y * z.y) + c.x;
        float y = (z.y * z.x + z.x * z.y) + c.y;
        last_itr = i;

        if((x * x + y * y) > 2.0) break;
        z.x = x;
        z.y = y;
    }

    float t = float(i) / float(itr);

    colorOut = map_to_color(float(t), last_itr);
}
