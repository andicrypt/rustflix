use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug,Parser)]  
#[clap(author, version, about)]
pub struct RustflixArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType, 
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    // Create, update, delete or show users
    User(UserCommand),

    // Create, update, delete or show videos
    Video(VideoCommand),

    // Create or show views
    View(ViewCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// Create a new user
    Create(CreateUser),

    /// Update an existing user
    Update(UpdateUser),

    /// Delete an user
    Delete(DeleteUser),

    /// Show all users
    Show
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct UpdateUser {
    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct DeleteUser {
    /// The ID of the entity to delete
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct VideoCommand {
    #[clap(subcommand)]
    pub command: VideoSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum VideoSubcommand {
    /// Create a new video
    Create(VideoDetails),

    /// Update an existing video
    Update(VideoDetails),

    /// Delete a video
    Delete(DeleteUser),

    /// Show all videos
    Show
}

#[derive(Debug, Args)]
pub struct VideoDetails {
    /// The title of the video
    pub title: String,

    /// The URL of the video
    pub url: String,
}

#[derive(Debug, Args)]
pub struct DeleteVideo {
    /// The ID of the video to delete
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct ViewCommand {
    #[clap(subcommand)]
    pub command: ViewSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ViewSubcommand {
    /// Create a new view
    Create(CreateView),

    /// Show all views
    Show
}

#[derive(Debug, Args)]
pub struct CreateView {
    /// The ID of the video
    pub video_id: i32,

    /// The ID of the user
    pub user_id: i32,
}
