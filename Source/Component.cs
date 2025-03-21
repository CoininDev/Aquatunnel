using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;

namespace Aquatunnel;

public abstract class Component {
    protected Entity entity;
    public Component(Entity entity){
        this.entity = entity;
    }
    public abstract void Init();
    public abstract void Update(GameTime time);
    public abstract void Draw(SpriteBatch batch);
}
