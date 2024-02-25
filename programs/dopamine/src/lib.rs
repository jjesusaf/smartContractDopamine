use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke};
use anchor_lang::solana_program::system_instruction;

declare_id!("Fg6PaF8..."); // Reemplaza esto con el ID de tu programa

#[program]
pub mod tu_proyecto {
    use super::*;
    pub fn comprar_articulo(ctx: Context<ComprarArticulo>, cantidad: u64) -> Result<()> {
        let articulo = &mut ctx.accounts.articulo;
        require!(cantidad <= articulo.cantidad, ErrorCode::CantidadInsuficiente);
        
        let costo_total = articulo.precio_sol.checked_mul(cantidad).ok_or(ErrorCode::Overflow)?;

        // Ejemplo de transferencia de SOL directamente, reemplaza esto con lógica de token si usas SPL tokens
        **ctx.accounts.vendedor.try_borrow_mut_lamports()? += costo_total;
        **ctx.accounts.comprador.try_borrow_mut_lamports()? -= costo_total;

        articulo.cantidad -= cantidad;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ComprarArticulo<'info> {
    #[account(mut)]
    pub comprador: Signer<'info>,
    #[account(mut)]
    pub articulo: Account<'info, Articulo>,
    #[account(mut)]
    pub vendedor: AccountInfo<'info>,
}

#[account]
pub struct Articulo {
    pub id: u64,
    pub nombre: String,
    pub coleccion: String,
    pub precio_sol: u64, // Asume precio en lamports
    pub cantidad: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("La cantidad solicitada supera la cantidad disponible")]
    CantidadInsuficiente,
    #[msg("Error de overflow en el cálculo del costo")]
    Overflow,
}
