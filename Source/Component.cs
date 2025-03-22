using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;

namespace Aquatunnel;

public abstract class Component {
    public Entity Entity;
    public abstract void Init();
    public abstract void Update(GameTime time);
    public abstract void Draw(SpriteBatch batch);
}
