using System;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;

namespace Aquatunnel;

public class PlayerComp : Component
{
    public float speed = 30f;
    public override void Draw(SpriteBatch batch) {}
    public override void Init() {}
    public override void Update(GameTime time) {
        var kstate = Keyboard.GetState();

        var dx = -Convert.ToInt16(kstate.IsKeyDown(Keys.A)) +Convert.ToInt16(kstate.IsKeyDown(Keys.D));
        var dy = -Convert.ToInt16(kstate.IsKeyDown(Keys.W)) +Convert.ToInt16(kstate.IsKeyDown(Keys.S));
        var dir = new Vector2(dx,dy);
        dir.Normalize();
        entity.Position += dir * speed * time.ElapsedGameTime.Milliseconds;
    }
}