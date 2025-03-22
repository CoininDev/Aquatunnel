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
        this.Texture = Entity.Scene.Game.Content.Load<Texture2D>(this.TextureName);
    }
    
    public override void Draw(SpriteBatch batch) {
        batch.Draw(
            Texture, 
            Entity.Space.Position, 
            null, 
            Color.White, 
            Entity.Space.Rotation, 
            Vector2.Zero, 
            Entity.Space.Scale, 
            SpriteEffects.None, 0
        );
    }
    
    public override void Update(GameTime time) {}
}
