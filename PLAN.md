modules{
    
    sound-engine
    input-handler
    ui-engine
    game-logic
    renderer

}

game-logic:
    startup:
        initialize variables
        parse world file
        generate step mesh
        load sounds
        load ui
        load textures

    loop:
        handle input:
            direction:
                Forward
                Backward
                Left
                Right
                LStrafe
                RStrafe
                    (Move in step-mesh)
            sprint
            inventory
            inspect
            interact
            speak
            KILL
            inventory

        render:

            get camera position/rotation
            get scene vertices*camPosRot
            skip hidden vertices
            calculate Z-buffer
            construct faces
            get triangels
            affine texturing
            shading
            rastorization
            draw framebuffer
            switch framebuffer

        ui:
            HP-bar
            SP-bar
            self-feeling
            weapon
            dialog windows

    background loop:
        sound:
            music
            ambient
            interaction-sound
            misc



WDF - world description file



Cell_Map:
    2D-array of cells: [hight: u8, id: u8]
        hight - point in Y-coordinates
        id - id of entity on this cell

World:
    Polygons: [[coordinates], [texture]]
    Cells_allighn: [?]

Character:
    Position: Vector2
    Rotation: Vector2
    HealthPoints: u8
    SprintPoints: u8
    Self-feeling: u8
    Inventory: [?]
    FlashlightOn: bool
    
Entities:
    id: u16
    HP: u8
    Drop: [?]
    Model: [?]
    Name: [?]
    DialogID: u16
    InspectID: u16

Items:
    id: u16
    name: [?]
    descriptionID: u16
    interactResult: u16 (key, medkit, etc)
    icon: [?]

Dialogs:
    id: u16
    nextDialogId: u16
    text: [?]

#Maybe just reuse dialog?
Description (Inspection):
    id: u16
    text: [?]    