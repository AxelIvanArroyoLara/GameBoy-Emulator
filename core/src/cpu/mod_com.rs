// Importa todo lo público que exista dentro del módulo `utils`.
//
// `crate` significa "desde la raíz de este crate/proyecto".
// `utils` sería un archivo o módulo llamado utils.
// `*` significa "importa todo lo público".
//
// Este código asume que ahí tienes cosas como:
// - merge_bytes(high: u8, low: u8) -> u16
// - trait/métodos high_byte() y low_byte() para u16
use crate::utils::*;

// `pub struct Cpu` define una estructura pública llamada Cpu.
//
// Una struct en Rust es parecida a una clase simple de otros lenguajes:
// guarda datos relacionados en un solo tipo.
//
// En este caso representa el estado interno del CPU del Game Boy.
pub struct Cpu {
    // Program Counter.
    // Guarda la dirección de memoria de la próxima instrucción.
    //
    // u16 = unsigned 16-bit integer.
    // Puede guardar valores de 0 a 65535.
    pc: u16,

    // Stack Pointer.
    // Apunta a la parte actual de la pila.
    sp: u16,

    // Registros de 8 bits del CPU.
    //
    // u8 = unsigned 8-bit integer.
    // Puede guardar valores de 0 a 255.
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,

    // Registro F.
    // En el Game Boy, F guarda las flags:
    // Z N H C en los bits altos.
    //
    // Los 4 bits bajos normalmente deben quedar en 0.
    f: u8,

    h: u8,
    l: u8,
}

// `impl Cpu` significa:
// "aquí voy a definir funciones/métodos asociados al tipo Cpu".
//
// Todo lo que esté dentro de este bloque pertenece a Cpu.
impl Cpu {
    // Constructor de Cpu.
    //
    // `pub` significa que esta función se puede usar desde otros módulos.
    //
    // `fn new()` define una función llamada new.
    //
    // `-> Self` significa que devuelve una instancia del mismo tipo.
    // Dentro de `impl Cpu`, `Self` significa `Cpu`.
    pub fn new() -> Self {
        // Para crear una struct, se escribe:
        //
        // Tipo {
        //     campo: valor,
        // }
        //
        // Aquí usamos `Self` en lugar de `Cpu`.
        Self {
            pc: 0x0000,
            sp: 0x0000,

            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00,
        }
    }

    // Lee un registro de 8 bits.
    //
    // `&self` significa que este método recibe una referencia inmutable
    // al Cpu actual.
    //
    // Inmutable significa que este método puede leer `self.a`, `self.b`, etc.,
    // pero NO puede modificarlos.
    //
    // `r: Regs` significa que recibe un parámetro llamado r
    // cuyo tipo es Regs.
    //
    // `-> u8` significa que devuelve un valor de tipo u8.
    pub fn get_r8(&self, r: Regs) -> u8 {
        // `match` es parecido a un switch, pero más poderoso.
        //
        // Evalúa el valor de `r` y ejecuta el caso correspondiente.
        match r {
            // Si r es Regs::A, devuelve self.a.
            //
            // En Rust, si una expresión no termina en `;`,
            // esa expresión se devuelve automáticamente.
            Regs::A => self.a,
            Regs::B => self.b,
            Regs::C => self.c,
            Regs::D => self.d,
            Regs::E => self.e,
            Regs::F => self.f,
            Regs::H => self.h,
            Regs::L => self.l,
        }
    }

    // Escribe en un registro de 8 bits.
    //
    // Aquí usamos `&mut self` en vez de `&self`.
    //
    // `&mut self` significa que este método puede modificar el Cpu.
    // Es obligatorio porque hacemos cosas como `self.a = val`.
    //
    // Esta función no devuelve nada.
    // En Rust, si no pones `-> Tipo`, la función devuelve `()`,
    // que significa "unit", algo parecido a void.
    pub fn set_r8(&mut self, r: Regs, val: u8) {
        match r {
            // Cada rama modifica el registro correspondiente.
            Regs::A => self.a = val,
            Regs::B => self.b = val,
            Regs::C => self.c = val,
            Regs::D => self.d = val,
            Regs::E => self.e = val,

            // El registro F del Game Boy solo usa los 4 bits altos.
            //
            // `val & 0xF0` limpia los 4 bits bajos.
            //
            // Ejemplo:
            // val      = 1011_0110
            // 0xF0     = 1111_0000
            // resultado= 1011_0000
            Regs::F => self.f = val & 0xF0,

            Regs::H => self.h = val,
            Regs::L => self.l = val,
        }
    }

    // Lee un registro de 16 bits.
    //
    // Algunos registros de 16 bits se forman juntando dos registros de 8 bits.
    //
    // AF = A como byte alto + F como byte bajo
    // BC = B como byte alto + C como byte bajo
    // DE = D como byte alto + E como byte bajo
    // HL = H como byte alto + L como byte bajo
    // SP = ya existe directamente como u16
    pub fn get_r16(&self, r: Regs16) -> u16 {
        match r {
            // merge_bytes(high, low) debería crear un u16 así:
            //
            // high = 0x12
            // low  = 0x34
            // resultado = 0x1234
            Regs16::AF => merge_bytes(self.a, self.f),
            Regs16::BC => merge_bytes(self.b, self.c),
            Regs16::DE => merge_bytes(self.d, self.e),
            Regs16::HL => merge_bytes(self.h, self.l),

            // SP ya es de 16 bits, así que no se combina con nada.
            Regs16::SP => self.sp,
        }
    }

    // Escribe un registro de 16 bits.
    //
    // Como va a modificar el CPU, necesita `&mut self`.
    pub fn set_r16(&mut self, r: Regs16, val: u16) {
        // Aquí separamos el valor de 16 bits en dos valores de 8 bits.
        //
        // Ejemplo:
        // val = 0x1234
        // high = 0x12
        // low  = 0x34
        //
        // Esto asume que `high_byte()` y `low_byte()` vienen de tu módulo utils.
        let high = val.high_byte();
        let low = val.low_byte();

        match r {
            Regs16::AF => {
                // Para AF:
                // A recibe el byte alto.
                // F recibe el byte bajo.
                //
                // Nota: set_r8 para F ya aplica `& 0xF0`.
                self.set_r8(Regs::A, high);
                self.set_r8(Regs::F, low);
            }

            Regs16::BC => {
                self.set_r8(Regs::B, high);
                self.set_r8(Regs::C, low);
            }

            Regs16::DE => {
                self.set_r8(Regs::D, high);
                self.set_r8(Regs::E, low);
            }

            Regs16::HL => {
                self.set_r8(Regs::H, high);
                self.set_r8(Regs::L, low);
            }

            Regs16::SP => {
                // SP no se divide en dos registros internos.
                // Se guarda directamente como u16.
                self.sp = val;
            }
        }
    }

    // Lee una flag del registro F.
    //
    // Devuelve bool:
    // - true si la flag está activa
    // - false si la flag está apagada
    pub fn get_flag(&self, f: Flags) -> bool {
        match f {
            // `&` es AND binario.
            //
            // self.f & 0b1000_0000 revisa si el bit Z está activo.
            //
            // Si el resultado es distinto de 0, entonces la flag está activa.
            Flags::Z => (self.f & 0b1000_0000) != 0,
            Flags::N => (self.f & 0b0100_0000) != 0,
            Flags::H => (self.f & 0b0010_0000) != 0,
            Flags::C => (self.f & 0b0001_0000) != 0,
        }
    }

    // Modifica una flag del registro F.
    //
    // `f: Flags` indica cuál flag queremos modificar.
    // `val: bool` indica si queremos activarla o apagarla.
    pub fn set_flag(&mut self, f: Flags, val: bool) {
        if val {
            // Si val es true, activamos la flag.
            match f {
                // `|=` hace OR binario y guarda el resultado.
                //
                // self.f |= 0b1000_0000
                //
                // Esto prende el bit Z sin tocar los demás bits.
                Flags::Z => self.f |= 0b1000_0000,
                Flags::N => self.f |= 0b0100_0000,
                Flags::H => self.f |= 0b0010_0000,
                Flags::C => self.f |= 0b0001_0000,
            }
        } else {
            // Si val es false, apagamos la flag.
            match f {
                // `&=` hace AND binario y guarda el resultado.
                //
                // Para apagar un bit, usamos una máscara con ese bit en 0
                // y los demás en 1.
                //
                // Ejemplo para Z:
                //
                // self.f &= 0b0111_1111
                //
                // Esto apaga el bit 7 y deja intactos los demás.
                Flags::Z => self.f &= 0b0111_1111,
                Flags::N => self.f &= 0b1011_1111,
                Flags::H => self.f &= 0b1101_1111,
                Flags::C => self.f &= 0b1110_1111,
            }
        }

        // Seguridad extra para el registro F del Game Boy:
        // los 4 bits bajos siempre deben quedar en 0.
        self.f &= 0xF0;
    }
}

// `enum` define un tipo que puede tener varios valores posibles.
//
// Este enum representa los registros de 8 bits.
//
// `#[derive(Copy, Clone)]` le dice a Rust que este tipo puede copiarse
// fácilmente.
//
// Sin esto, al pasar un Regs a una función podrías "moverlo".
// Con Copy, se copia como un número pequeño.
#[derive(Copy, Clone)]
pub enum Regs {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

// Este enum representa registros de 16 bits.
//
// AF, BC, DE y HL son pares de registros.
// SP es el Stack Pointer.
#[derive(Copy, Clone)]
pub enum Regs16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

// Este enum representa las flags del CPU.
//
// En el Game Boy, el registro F usa estas flags:
//
// Z = Zero flag
// N = Subtract flag
// H = Half Carry flag
// C = Carry flag
pub enum Flags {
    Z,
    N,
    H,
    C,
}