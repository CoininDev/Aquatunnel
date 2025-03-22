using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using System.Collections.Generic;

namespace Aquatunnel;

public class Entity {
    public Space Space;
    public List<Component> Components;
    public Scene Scene;
    
    public Entity(Space space, params Component[] components) {
        this.Space = space;
        this.Components = new List<Component>(components);
        foreach(var comp in this.Components) {
           comp.Entity = this;
        }
    }

    public Entity(params Component[] components) 
    : this(new Space(0,0,1,1,0), components)
    {}

    public void Init(Scene scene) {
        this.Scene = scene;

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

    private void assignComponents(){
        
    }
}

public class Space {
    public Vector2 Position;
    public Vector2 Scale;
    public float Rotation;

    public Space(float x, float y, float w, float h, float r){
        this.Position = new Vector2(x,y);
        this.Scale = new Vector2(w,h);
        this.Rotation = r;
    }

    public Space(Vector2 pos, Vector2 scale, float r){
        this.Position = pos;
        this.Scale = scale;
        this.Rotation = r;
    }
}
