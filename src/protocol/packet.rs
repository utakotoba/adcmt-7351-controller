use anyhow::{Ok, Result};

use crate::protocol::MAX_CMD_LEN;

/// Packet encoder and decoder
pub struct Packet;

#[allow(dead_code)]
impl Packet {
    /// Encode a write request into a packet
    pub fn encode_write(command_str: &str, sequence: u8) -> Result<Vec<u8>> {
        if command_str.len() > MAX_CMD_LEN {
            anyhow::bail!("Command too long (max {} bytes)", MAX_CMD_LEN);
        }

        if command_str.is_empty() {
            anyhow::bail!("Command cannot be empty");
        }

        // Round up (command + newline) to next 4-byte boundary
        let aligned_len = ((command_str.len() + 1 + 3) & 0xFFFFFFFC) as u32;

        // Lower 32 bits of header: 01 [seq] [~seq] 00
        let lower_header = ((!sequence as u32 & 0xFF) << 16) | ((sequence as u32 & 0xFF) << 8) | 1;

        // Upper 32 bits of header: aligned_len
        let upper_header = aligned_len;

        // Total length of packet: 12 (header) + aligned_len (data)
        let total_size = 12 + aligned_len as usize;
        let mut packet = vec![0u8; total_size]; // zero value init

        // Copy header data into packet (little-endian)
        packet[0..4].copy_from_slice(&lower_header.to_le_bytes());
        packet[4..8].copy_from_slice(&upper_header.to_le_bytes());

        // Header surfix
        packet[8] = 1;
        packet[9] = 0;
        packet[10] = 0;
        packet[11] = 0;

        // Copy command data into packet
        packet[12..12 + command_str.len()].copy_from_slice(command_str.as_bytes());

        // Append newline
        packet[12 + command_str.len()] = 0x0A;

        Ok(packet)
    }

    /// Encode a read request into a packet
    pub fn encode_read(sequence: u8) -> Vec<u8> {
        // No data in read request, pad to 4 bytes
        let aligned_len = 4u32;
        let total_size = 12 + aligned_len as usize;
        let mut packet = vec![0u8; total_size]; // zero value init

        // Lower 32 bits of header: 02 [seq] [~seq] 00
        let lower_header = ((!sequence as u32 & 0xFF) << 16) | ((sequence as u32 & 0xFF) << 8) | 2;

        // Upper 32 bits of header: aligned_len
        let upper_header = aligned_len;

        // Copy header data into packet (little-endian)
        packet[0..4].copy_from_slice(&lower_header.to_le_bytes());
        packet[4..8].copy_from_slice(&upper_header.to_le_bytes());

        // Header surfix
        packet[8] = 0;
        packet[9] = 0;
        packet[10] = 0;
        packet[11] = 0;

        packet
    }

    /// Devode a read response packet
    pub fn decode_read(buffer: &[u8]) -> Result<Vec<u8>> {
        if buffer.is_empty() {
            return Ok(Vec::new());
        }

        let (data_start, data_size) = if buffer.len() >= 12 && buffer[0] == 0x02 {
            // Extract upper header
            let response_data_len =
                u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]) as usize;

            let data_start = 12;
            let mut data_size = buffer.len() - 12;

            if response_data_len > 0 && response_data_len < data_size {
                data_size = response_data_len;
            }

            (data_start, data_size)
        } else {
            // Raw data (no header)
            (0, buffer.len())
        };

        // Extract data
        let mut data = buffer[data_start..data_start + data_size].to_vec();

        // Strip trailing CR/LF
        while let Some(&b) = data.last() {
            if b == b'\r' || b == b'\n' {
                data.pop();
            } else {
                break;
            }
        }

        Ok(data)
    }
}
