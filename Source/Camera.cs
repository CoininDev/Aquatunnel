using Microsoft.Xna.Framework;

namespace Aquatunnel;


public class Camera {
    public Space Space;
}

public class FollowCamera : Camera{
    public Entity Target;

    public void StepFollow(float smoothness){
        if (Target == null) return;

        this.Space.Position = Vector2.Lerp(this.Space.Position, Target.Space.Position, smoothness);
    }
}


