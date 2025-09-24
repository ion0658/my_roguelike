use bevy::{color::palettes::tailwind::*, prelude::*};

#[derive(EntityEvent)]
pub struct ButtonClicked(Entity);
impl From<Entity> for ButtonClicked {
    fn from(entity: Entity) -> Self {
        ButtonClicked(entity)
    }
}

const BUTTON_BORDER_COLOR: Color = Color::Srgba(GRAY_500);
const BUTTON_BACKGROUND_COLOR: Color = Color::Srgba(GRAY_900);
const BUTTON_HOVERED_BORDER_COLOR: Color = Color::Srgba(GRAY_400);
const BUTTON_HOVERED_BACKGROUND_COLOR: Color = Color::Srgba(GRAY_800);
const BUTTON_PRESSED_BORDER_COLOR: Color = Color::Srgba(GRAY_300);
const BUTTON_PRESSED_BACKGROUND_COLOR: Color = Color::Srgba(GRAY_700);

#[derive(Component)]
pub struct UiButton;

pub fn create_button(label: &str) -> impl Bundle {
    use bevy::color::palettes::tailwind::*;
    (
        UiButton,
        Button,
        Node {
            min_width: px(200.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(px(5.)),
            border: UiRect::all(px(2.5)),
            ..Default::default()
        },
        BorderColor::all(BUTTON_BORDER_COLOR),
        BorderRadius::all(px(5.)),
        BackgroundColor(BUTTON_BACKGROUND_COLOR),
        children![(
            Text(label.into()),
            TextColor(Color::Srgba(RED_600)),
            TextLayout {
                justify: Justify::Center,
                ..Default::default()
            }
        )],
    )
}

pub fn button_interaction_event(
    mut commands: Commands,
    mut buttons: Query<
        (Entity, &Interaction, &mut BorderColor, &mut BackgroundColor),
        (Changed<Interaction>, With<UiButton>),
    >,
) {
    log::trace!("button_interaction_event");
    for (entity, interaction, mut border, mut bg) in &mut buttons {
        match interaction {
            Interaction::Pressed => {
                log::trace!("Button pressed: {:?}", entity);
                *border = BorderColor::all(BUTTON_PRESSED_BORDER_COLOR);
                bg.0 = BUTTON_PRESSED_BACKGROUND_COLOR;
                commands.entity(entity).trigger(ButtonClicked);
            }
            Interaction::Hovered => {
                log::trace!("Button hovered: {:?}", entity);
                *border = BorderColor::all(BUTTON_HOVERED_BORDER_COLOR);
                bg.0 = BUTTON_HOVERED_BACKGROUND_COLOR;
            }
            Interaction::None => {
                log::trace!("Button none: {:?}", entity);
                *border = BorderColor::all(BUTTON_BORDER_COLOR);
                bg.0 = BUTTON_BACKGROUND_COLOR;
            }
        }
    }
}
