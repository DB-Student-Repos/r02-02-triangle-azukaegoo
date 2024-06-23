pub struct Triangle {
    sides: [u64; 3]
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        /* unimplemented!("Construct new Triangle from following sides: {sides:?}. Return None if the sides are invalid.");*/
        let [a, b, c] = sides;
        if a > 0 && b > 0 && c > 0 &&
            a + b > c &&
            b + c > a &&
            a + c > b {
            Some(Triangle { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        /* unimplemented!("Determine if the Triangle is equilateral."); */
        let [a, b, c] = self.sides;
        self.sides.iter().all(|&side| side > 0) && a == b && b == c

    }

    pub fn is_scalene(&self) -> bool {
        /* unimplemented!("Determine if the Triangle is scalene."); */
        let [a, b, c] = self.sides;
        a != b && b != c && a != c
    }

    pub fn is_isosceles(&self) -> bool {
        /* unimplemented!("Determine if the Triangle is isosceles."); */
        let [a, b, c] = self.sides;
        a == b || a == c || b == c
    }
}
