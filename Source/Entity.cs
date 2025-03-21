using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using System.Collections.Generic;

namespace Aquatunnel;

public class Entity {
    public Space Space;
    public List<Component> Components;
    
    public Entity(List<Component> components) {
        this.Components = components;
    }

    public void Init() {
        foreach(Component comp in this.Components) {
            comp.Init();
        }
    }

    public void Step(GameTime time) {
        foreach(Component comp in this.Components) {
            comp.Update(time);
        }
    }

    public void Draw(SpriteBatch batch) {
        foreach(Component comp in this.Components) {
            comp.Draw(batch);
        }
    }

    public void RemoveComponent<T>() where T : Component {
        Components.RemoveAll(c => c is T);
    }

    public void AddComponent(Component comp) {
        if (comp == null) return;
        Components.Add(comp);
    }
}

public struct Space {
    public Vector2 Position;
    public Vector2 Scale;
    public float Rotation;
}
