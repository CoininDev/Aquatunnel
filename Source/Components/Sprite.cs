using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;

namespace Aquatunnel;

public class SpriteComp : Component{
    public Texture2D Texture;
    
    public SpriteComp(Entity entity, Texture2D texture) : base(entity){
        this.Texture = texture;
    }

    public override void Draw(SpriteBatch batch) 
    {
        batch.Draw(Texture, entity.Space.Scale, Color.White);
    }
    public override void Init() {}
    public override void Update(GameTime time) {}
}
