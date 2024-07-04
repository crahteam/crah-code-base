use thiserror::Error;
use wgpu::RequestDeviceError;
#[derive(Error, Debug)]
#[error("The device request failed. {0}")]
pub struct DeviceError(pub RequestDeviceError);

#[derive(Error, Debug)]
pub enum InteractionError {
    #[error("No entity was found at the initiator's role")]
    InitiatorNotFound,
    #[error("No entity was found at the target's role")]
    TargetNotFound
}

#[derive(Error, Debug)]
#[error("The terrain generation failed")]
pub struct TerrainGenerationError;

