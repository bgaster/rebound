var svgpath = require('svgpath');


//M60,60 L60,150 L150,150 L240,150 L240,240 Z
var transformed = svgpath('M60,60 L60,150 L150,150 L240,150 L240,240')
		    .unarc()
                    .scale(0.13)
                    .translate(312,680)
                    //.rel()
                    //.round(1)
                    .toString();


console.log(transformed);
