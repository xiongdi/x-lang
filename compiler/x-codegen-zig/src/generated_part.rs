            x_lir::Expression::InitializerList(inits) => {
                // In Zig, this becomes .{ ... }
                let mut init_strs = Vec::new();
                for init in inits {
                    init_strs.push(self.emit_lir_initializer(init)?);
                }
                Ok(format!".{{ {} }}", init_strs.join(", "))
            }
            x_lir::Expression::CompoundLiteral(ty, inits) => {
                let ty_str = self.emit_lir_type(ty);
                let mut init_strs = Vec::new();
                for init in inits {
                    init_strs.push(self.emit_lir_initializer(init)?);
                }
                Ok(format!("{} {{ {} }}", ty_str, init_strs.join(", ")))
            }
            _ => {
                // Handle other expression types as needed
                Ok("/* unimplemented expression */".to_string())
            }
        }
    }

    /// 发出初始化器（用于复合字面量）
    fn emit_lir_initializer(&mut self, init: &x_lir::Initializer) -> ZigResult<String> {
        match init {
            x_lir::Initializer::Expression(expr) => self.emit_lir_expression(expr),
            x_lir::Initializer::List(list) => {
                let mut items = Vec::new();
                for i in list {
                    items.push(self.emit_lir_initializer(i)?);
                }
                Ok(format!".{{ {} }}", items.join(", "))
            }
            x_lir::Initializer::Named(name, init) => {
                let init_str = self.emit_lir_initializer(init)?;
                Ok(format!".{} = {}", name, init_str)
            }
            x_lir::Initializer::Indexed(idx, init) => {
                let idx_str = self.emit_lir_expression(idx)?;
                let init_str = self.emit_lir_initializer(init)?;
                Ok(format!("[{}] = {}", idx_str, init_str))
            }
        }
    }
