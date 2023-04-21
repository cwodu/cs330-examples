use crate::flooring::*;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display; //,Formatter,Result};

//------------------------------------------------------------------------------
#[derive(Debug)]
pub enum BuildError<'a> {
    GenericError(&'a str),
}

/*
impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::GenericError(err)
    }
}
*/

impl<'a> fmt::Display for BuildError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            BuildError::GenericError(description) => {
                write!(f, "{:?}", description)
            }
        }
    }
}
//------------------------------------------------------------------------------

#[derive(Clone)]
pub struct DimensionSet {
    pub length: f64,
    pub width: f64,
}

impl DimensionSet {
    pub fn new(l: f64, w: f64) -> Self {
        DimensionSet {
            length: l,
            width: w,
        }
    }
}

impl Default for DimensionSet {
    fn default() -> Self {
        DimensionSet::new(1f64, 1f64)
    }
}

//------------------------------------------------------------------------------
#[derive(Clone)]
pub struct Room {
    pub name: String,
    pub dimensions: DimensionSet,
    pub flooring: Flooring,
}

impl Room {
    /// Set the name using the builder pattern.
    ///
    /// # Arguments
    ///  * `nme` - room name
    ///
    pub fn with_name(mut self, nme: &str) -> Self {
        self.name = nme.to_string();

        self
    }

    /// Set the Flooring using the builder pattern.
    ///
    /// # Arguments
    ///  * `nme` - flooring type name
    ///  * `unit_c` - unit cost
    ///
    pub fn with_flooring(mut self, nme: &str, unit_c: f64) -> Self {
        self.flooring.type_name = nme.to_string();
        self.flooring.unit_cost = unit_c;

        self
    }

    /// Set the Flooring using the builder pattern.
    ///
    /// # Arguments
    ///  * `l` - length
    ///  * `w` - width
    ///
    pub fn with_dimensions(mut self, l: f64, w: f64) -> Self {
        self.dimensions.length = l;
        self.dimensions.width = w;

        self
    }

    /// Set the flooring.
    ///
    /// # Arguments
    ///  * `nme` - flooring type name
    ///  * `unit_c` - unit cost
    ///
    pub fn set_flooring(&mut self, nme: &str, unit_c: f64) {
        self.flooring.type_name = nme.to_string();
        self.flooring.unit_cost = unit_c;
    }

    /// Compute the area of flooring for a room.
    pub fn area(&self) -> f64 {
        self.dimensions.width * self.dimensions.length
    }

    /// Compute the flooring cost based on `self.area()` and unit cost.
    pub fn flooring_cost(&self) -> f64 {
        self.area() * self.flooring.unit_cost
    }
}

impl Default for Room {
    fn default() -> Self {
        Room {
            name: "Generic".to_string(),
            dimensions: Default::default(),
            flooring: Default::default(),
        }
    }
}

impl Display for Room {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Room ({})", self.name);
        writeln!(f, "  {:<6}: {:>8.1}", "Length", self.dimensions.length);
        writeln!(f, "  {:<6}: {:>8.1}", "Width", self.dimensions.width);
        writeln!(f, "  {:<6}: {:>8.1}", "Area", self.area());
        writeln!(f);
        writeln!(f, "  Flooring  : {}", self.flooring.type_name);
        writeln!(f, "  Unit Cost : $ {:>8.2}", self.flooring.unit_cost);
        writeln!(f, "  Total Cost: $ {:>8.2}", self.flooring_cost())
    }
}

impl PartialOrd for Room {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        if self.name.eq(&rhs.name) {
            self.area().partial_cmp(&rhs.area())
        } else {
            self.name.partial_cmp(&rhs.name)
        }
    }
}

impl PartialEq for Room {
    fn eq(&self, rhs: &Self) -> bool {
        return self.name.eq(&rhs.name) && self.area().eq(&rhs.area());
    }
}

//------------------------------------------------------------------------------
pub struct RoomBuilder<'a> {
    name: Option<&'a str>,
    length: Option<f64>,
    width: Option<f64>,
    flooring_name: Option<&'a str>,
    unit_cost: Option<f64>,
}

impl<'a> RoomBuilder<'a> {
    pub fn new() -> Self {
        RoomBuilder {
            name: None,
            length: None,
            width: None,
            flooring_name: None,
            unit_cost: None,
        }
    }

    pub fn with_name(mut self, nme: &'a str) -> Self {
        self.name = Some(nme);

        self
    }

    pub fn with_flooring(mut self, nme: &'a str, unit_c: f64) -> Self {
        self.flooring_name = Some(nme);
        self.unit_cost = Some(unit_c);

        self
    }

    pub fn with_dimensions(mut self, l: f64, w: f64) -> Self {
        self.length = Some(l);
        self.width = Some(w);

        self
    }

    pub fn build(self) -> Result<Room, BuildError<'a>> {
        if self.name.is_none() {
            return Err(BuildError::GenericError("Name can not be blank"));
        }

        let room = Room {
            name: self.name.unwrap().to_owned(),
            dimensions: DimensionSet::new(self.length.unwrap(), self.width.unwrap()),
            flooring: Flooring {
                type_name: self.flooring_name.unwrap().to_owned(),
                unit_cost: self.unit_cost.unwrap(),
            },
        };

        Ok(room)
    }
}

impl From<(f64, f64)> for DimensionSet {
    fn from(dims: (f64, f64)) -> Self {
        DimensionSet::new(dims.0, dims.1)
    }
}
