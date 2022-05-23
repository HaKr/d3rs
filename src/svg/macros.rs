use crate::LengthOrPercentage;
use std::fmt::Display;

macro_rules! global_attributes {
    ($structure:ty) => {
        impl $structure {
            pub fn with_id<CT>(mut self, class: CT) -> Self
            where
                CT: Display,
            {
                let s = format!("{}", class);
                self.id = Some(s);

                self
            }

            pub fn with_class<CT>(mut self, class: CT) -> Self
            where
                CT: Display,
            {
                let s = format!("{}", class);
                self.class = Some(s);

                self
            }

            pub fn with_meta<CT>(mut self, meta: CT) -> Self
            where
                CT: Display,
            {
                self.meta = Some(format!("{}", meta));

                self
            }
        }
    };
}

macro_rules! has_children {
    ($structure:ty) => {
        impl $structure {
            pub fn add<CT>(&mut self, child: CT)
            where
                CT: Into<XMLElement>,
            {
                let child_element: XMLElement = child.into();
                self.items.push(child_element);
            }
        }
    };
}

#[macro_export]
macro_rules! svg_args {

    ($var:ident style: $css:expr) => {
        $var = $var.with_css( $css );
    };

    ($var:ident $attr_name:ident: $attr_val:expr ) => {
        $crate::add_global_attribute!( $var $attr_name: $attr_val );
    };

}

#[macro_export]
macro_rules! svg {
    ( $width:expr, $height:expr $(, $($attr_name:ident: $attr_val:expr),*)? $(, [ $($child:expr),+ ])? ) => {{
        let mut svg = Document::new( ($width) as u16, ($height) as u16 );
        $($(
            $crate::svg_args!( svg $attr_name: $attr_val );
        )*)?

        $($(
            svg.add( $child );
        )+)?


        svg
    }};
}

#[macro_export]
macro_rules! group {
    ( $($($attr_name:ident: $attr_val:expr),+)? $(, [ $($child:expr),+ ])? ) => {{

        let mut grp = Group::default();
        $($(
            $crate::add_global_attribute!( grp $attr_name: $attr_val );
        )+)?

        $($(
            grp.add( $child );
        )+)?

        grp
    }};

    ( [ $($child:expr),+ ] ) => {{

        let mut grp = Group::default();
        $(
            grp.add( $child );
        )+

        grp
    }};}

#[macro_export]
macro_rules! add_global_attributes {
    ($svg:ident $($all:tt)*) => {{
        $crate::add_global_attributes_args!( @munch $svg $($all)* );
    }}
}

#[macro_export]

macro_rules! add_global_attribute {
    ($var:ident id: $id:expr) => {
        $var = $var.with_id($id);
    };

    ($var:ident class: $class:expr) => {
        $var = $var.with_class($class);
    };

    ($var:ident meta: $meta:expr) => {
        $var = $var.with_meta($meta);
    };

    ($var:ident $name:ident: $($ignored:tt)* ) => {
        compile_error!("argument not recognised");
    };
}

#[macro_export]
macro_rules! add_global_attributes_args {
    (@munch $var:ident ) => {};

    (@munch $var:ident $( $name:ident: $val:expr $(,)? )*) => {
        $(
            $crate::add_global_attributes_args!( $var <- $name: $val );
        )*
    };

    ($var:ident <- id: $id:expr) => {
        $var = $var.with_id($id) ;
    };

    ($var:ident  <- class: $class:expr) => {
        $var = $var.with_class( $class );
    };

    ($var:ident <- $name:ident: $($ignored:tt)* ) => {};
}

#[macro_export]
macro_rules! line {
    ($from:expr, $to:expr $(, $($attr_name:ident: $attr_val:expr),+)?) => {{
        #[allow(unused_mut)]
        let (x1,y1) = $from;
        let (x2,y2) = $to;

        let mut line = $crate::Line::new(
            LengthOrPercentage::new(x1),
            LengthOrPercentage::new(y1),
            LengthOrPercentage::new(x2),
            LengthOrPercentage::new(y2)
        );

        $($(
            $crate::add_global_attribute!( line $attr_name: $attr_val );
        )+)?

        line
    }}
}

#[macro_export]
macro_rules! circle {
    ($center:expr, $radius:expr $(, $($attr_name:ident: $attr_val:expr),+)?) => {{
        #[allow(unused_mut)]
        let (cx, cy) = $center;

        let mut circle = $crate::Circle::new(
            LengthOrPercentage::new(cx),
            LengthOrPercentage::new(cy),
            LengthOrPercentage::new($radius)
        );

        $($(
            $crate::add_global_attribute!( circle $attr_name: $attr_val );
        )+)?

        circle
    }}
}

#[derive(Default)]
pub struct TextArgs {
    pub id: Option<String>,
    pub class: Option<String>,
    pub meta: Option<String>,

    pub text: Option<String>,
    pub at: Option<(LengthOrPercentage, LengthOrPercentage)>,
    pub relative: Option<(LengthOrPercentage, LengthOrPercentage)>,
    pub rotate: Option<u16>,
}

global_attributes!(TextArgs);

#[macro_export]
macro_rules! text_args {
    (@munch $var:ident ) => {};

    (@munch $var:ident $name:ident: $coord:expr $(, $($rest:tt)+ )? ) => {
            $crate::text_args!( $var $name: $coord );
            $crate::text_args!( @munch $var $($($rest)+)? );
    };

    ($var:ident text: $value:expr ) => {
        $var.text = Some( format!("{}", $value) );
    };

    ($var:ident at: $coord:expr ) => {
        let coord = $coord;
        $var.at = Some(( LengthOrPercentage::new(coord.0), LengthOrPercentage::new(coord.1) ));
    };

    ($var:ident relative: $coord:expr ) => {
        let coord = $coord;
        $var.relative = Some(( LengthOrPercentage::new(coord.0), LengthOrPercentage::new(coord.1) ));
    };

    ($var:ident rotate: $angle:expr ) => {
        $var = $var.relative = Some( $angle as u16 );
    };

    ($var:ident $name:ident: $val:expr ) => {
        $crate::add_global_attribute!( $var $name: $val );
    };

}

#[macro_export]
macro_rules! text {
    ($($all:tt)*) => {{
        #[allow(unused_mut)]
        let mut args = $crate::TextArgs::default();
        $crate::text_args!( @munch args $($all)* );

        let mut txt = $crate::Text::new( args.text.expect("Text is not optional") );

        if let Some(at) = args.at {
            txt = txt.at( at.0, at.1 );
        } else if let Some( relative) = args.relative {
            txt = txt.relative( relative.0, relative.1 );
        }

        if let Some(id) = args.id {
            txt = txt.with_id( id );
        }

        if let Some(class) = args.class {
            txt = txt.with_class( class);
        }

        txt
    }}
}

#[macro_export]
macro_rules! expr_or {
    ($default:expr => $expr:expr $(, $rest:tt)* ) => {
        $expr
    };
    ($default:expr => ) => {
        $default
    };
}

#[macro_export]
macro_rules! vertical_ticks {
    ( $iter:expr
        $(, position: $position:expr)?
        $(, tick_before: $tick_before:expr)?
        $(, tick_after: $tick_after:expr)?
        $(, tick_label: $tick_label:expr)?
        $(, label_x_offset: $label_x_offset:expr )?
        $(, label_y_offset: $label_y_offset:expr )?
) => {
        $crate::ticks!( $iter, "vertical-ticks", true,
            $(, position: $position )?
            $(, tick_before: $tick_before )?
            $(, tick_after: $tick_after )?
            $(, tick_label: $tick_label)?
            $(, label_x_offset: $label_x_offset )?
            $(, label_y_offset: $label_y_offset )?
    )};
}

#[macro_export]
macro_rules! horizontal_ticks {
    ( $iter:expr
        $(, position: $position:expr)?
        $(, tick_before: $tick_before:expr)?
        $(, tick_after: $tick_after:expr)?
        $(, tick_label: $tick_label:expr)?
        $(, label_x_offset: $label_x_offset:expr )?
        $(, label_y_offset: $label_y_offset:expr )?
) => {
        $crate::ticks!( $iter, "horizontal-ticks", false,
            $(, position: $position )?
            $(, tick_before: $tick_before )?
            $(, tick_after: $tick_after )?
            $(, tick_label: $tick_label)?
            $(, label_x_offset: $label_x_offset )?
            $(, label_y_offset: $label_y_offset )?
    )};
}

#[macro_export]
macro_rules! ticks {
    ($iter:expr, $class:expr, $is_vertical:expr,
        $(, position: $position:expr)?
        $(, tick_before: $tick_before:expr)?
        $(, tick_after: $tick_after:expr)?
        $(, tick_label: $tick_label:expr)?
        $(, label_x_offset: $label_x_offset:expr )?
        $(, label_y_offset: $label_y_offset:expr )?
) => {{
        let mut grp = $crate::group!(class: $class );

        let base = ($crate::expr_or!( 0 => $($position)? ));
        let offset = $crate::expr_or!( 4 => $($tick_before)? );
        let base_start = if (base > offset) {base - offset} else {base} ;
        let base_end = (base + ($crate::expr_or!( 4 => $($tick_after)? ))  );

        let label_x_offset = $crate::expr_or!( 0 => $($label_x_offset)? );
        let label_y_offset = $crate::expr_or!( 0 => $($label_y_offset)? );

        for (domain, dimension) in $iter {
            let (from, to) = if $is_vertical {
                (( base_start, dimension ), ( base_end, dimension ))
            } else {
                (( dimension, base_start ), ( dimension, base_end ))
            };
            grp.add(d3rs::line!( from, to, class: "tick" ));

            if let Some(text) = $crate::expr_or!( None => $(($tick_label)(domain, dimension))? ) {
                let (text_x, text_y) = if $is_vertical {
                    (((base_start as isize) + label_x_offset) as usize, ((dimension as isize) + label_y_offset) as usize )
                } else {
                    (((dimension as isize) + label_x_offset) as usize, ((base_end as isize) + label_y_offset) as usize )
                };
                grp.add(d3rs::text!(text: text, at: (text_x, text_y), class: "tick-label" ));
            }
        }

        grp
    }};
}

#[macro_export]
macro_rules! plot {
    ($iter:expr, $plotter:expr $(, $($attr_name:ident: $attr_val:expr),+)?) => {{
        #[allow(unused_mut)]
        let mut chart = $crate::group!(class: "chart" );

        $($(
            $crate::add_global_attribute!( chart $attr_name: $attr_val );
        )+)?

        for (domain, dimension) in $iter {
            if let Some(result) = ($plotter)(domain, dimension) {
                chart.add( result );
            }
        }

        chart
    }}
}

#[macro_export]
macro_rules! horizontal_axis {
    ( $from: expr, $to:expr, $iter:expr
        $(, position: $position:expr)?
        $(, tick_before: $tick_before:expr)?
        $(, tick_after: $tick_after:expr)?
        $(, tick_label: $tick_label:expr)?
        $(, label_x_offset: $label_x_offset:expr )?
        $(, label_y_offset: $label_y_offset:expr )?
) => {
        d3rs::group!( class: "horizontal-axis", [
            $crate::ticks!( $iter, "horizontal-ticks", false,
                $(, position: $position )?
                $(, tick_before: $tick_before )?
                $(, tick_after: $tick_after )?
                $(, tick_label: $tick_label)?
                $(, label_x_offset: $label_x_offset )?
                $(, label_y_offset: $label_y_offset )?
            ),
            d3rs::line!( $from, $to )
        ])
    };
}

#[macro_export]
macro_rules! vertical_axis {
    ( $from: expr, $to:expr, $iter:expr
        $(, position: $position:expr)?
        $(, tick_before: $tick_before:expr)?
        $(, tick_after: $tick_after:expr)?
        $(, tick_label: $tick_label:expr)?
        $(, label_x_offset: $label_x_offset:expr )?
        $(, label_y_offset: $label_y_offset:expr )?
) => {
        d3rs::group!( class: "verticalal-axis", [
            $crate::ticks!( $iter, "vertical-ticks", true,
                $(, position: $position )?
                $(, tick_before: $tick_before )?
                $(, tick_after: $tick_after )?
                $(, tick_label: $tick_label)?
                $(, label_x_offset: $label_x_offset )?
                $(, label_y_offset: $label_y_offset )?
            ),
            d3rs::line!( $from, $to )
        ])
    };
}
