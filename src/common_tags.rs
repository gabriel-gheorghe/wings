use bevy::prelude::*;

/// A common tag used to specify which entity can be modified with
/// [`LayoutVisibility`](crate::prelude::LayoutVisibility)
///
/// ```
/// use bevy::prelude::*;
/// use wings::prelude::*;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(WingsPlugin)
///         .add_systems(Startup, startup)
///         .add_systems(Update, change_visibility)
///         .run();
/// }
///
/// fn startup(mut commands: Commands) {
///     commands.spawn(Camera2dBundle::default());
///
///     widget_tree! {
///         Scaffold {
///             child: Center {
///                 child: LayoutVisibility {
///                     tags: [Collapsible]
///                     child: Container {}
///                 }
///             }
///         }
///     }
/// }
///
/// fn change_visibility(
///     keyboard_input: Res<Input<KeyCode>>,
///     mut query: LayoutVisibilityQuery<Collapsible>,
/// ) {
///     if keyboard_input.just_pressed(KeyCode::Space) {
///         query.set_visibility(LayoutVisibility::Collapsed);
///     }
/// }
/// ```
#[derive(Component)]
pub struct Collapsible;