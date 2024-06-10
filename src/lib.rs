pub struct Triangle {
    sides: [u64; 3]
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        /* unimplemented!("Construct new Triangle from following sides: {sides:?}. Return None if the sides are invalid.");*/
        match sides {
            [side_one, side_two, side_three] if [side_one, side_two, side_three].iter().all(|&side| side > 0) &&
                side_one + side_two > side_three &&
                side_two + side_three > side_one &&
                side_one + side_three > side_two => {
                Some(Triangle { sides })
            }
            _ => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        /* unimplemented!("Determine if the Triangle is equilateral."); */
        match &self.sides {
            [side_one, side_two, side_three] if [side_one, side_two, side_three].iter().all(|&side| side > &0) &&
                side_one == side_two && side_two == side_three => true,
            _ => false
        }
    }

    pub fn is_scalene(&self) -> bool {
        /* unimplemented!("Determine if the Triangle is scalene."); */
        match &self.sides {
            [side_one, side_two, side_three] if side_one != side_two && side_two != side_three && side_one != side_three => true,
            _ => false
        }
    }

    pub fn is_isosceles(&self) -> bool {
        /* unimplemented!("Determine if the Triangle is isosceles."); */
        match &self.sides {
            [side_one, side_two, side_three] if side_one == side_two || side_one == side_three || side_two == side_three => true,
            _ => false
        }
    }
}
