pub struct Triangle {
    sides: [u64; 3]
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        /* unimplemented!("Construct new Triangle from following sides: {sides:?}. Return None if the sides are invalid.");*/
        let [side_one, side_two, side_three] = sides;
        if side_one > 0 && side_two > 0 && side_three > 0 &&
            side_one + side_two > side_three &&
            side_two + side_three > side_one &&
            side_one + side_three > side_two {
            Some(Triangle { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        /* unimplemented!("Determine if the Triangle is equilateral."); */
        let [side_one, side_two, side_three] = self.sides;
        self.sides.iter().all(|&side| side > 0) && side_one == side_two && side_two == side_three

    }

    pub fn is_scalene(&self) -> bool {
        /* unimplemented!("Determine if the Triangle is scalene."); */
        let [side_one, side_two, side_three] = self.sides;
        side_one != side_two && side_two != side_three && side_one != side_three
    }

    pub fn is_isosceles(&self) -> bool {
        /* unimplemented!("Determine if the Triangle is isosceles."); */
        let [side_one, side_two, side_three] = self.sides;
        side_one == side_two || side_one == side_three || side_two == side_three
    }
}
