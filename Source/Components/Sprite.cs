using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;

namespace Aquatunnel;

public class SpriteComp : Component{
    public string TextureName;
    public Texture2D Texture;
    
    public SpriteComp(string textureName) {
        this.TextureName = textureName;
    }

    public override void Init() {
        Entity.Game.Load<Texture2D>(this.TextureName);
    }
    
    public override void Draw(SpriteBatch batch) {
        batch.Draw(Texture, entity.Space.Scale, Color.White);
    }
    
    public override void Update(GameTime time) {}
}
