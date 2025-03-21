using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using System.Collections.Generic;

namespace Aquatunnel;

public class Scene {
    public List<Entity> Entities;

    public Scene(List<Entity> entities) {
        this.Entities = entities;
    }

    public void Init() {
        foreach(Entity entity in Entities){
            entity.Init();
        }
    }

    public void Step(GameTime time) {
        foreach(Entity entity in Entities){
            entity.Step(time);
        }
    }

    public void Draw(SpriteBatch batch) {
        foreach(Entity entity in Entities){
            entity.Draw(batch);
        }
    }
}
