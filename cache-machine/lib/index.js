var addon = require('../native');
const result = addon.opeator(1,new Date().getTime().toString()); //get


if (result) {
    let last = addon.opeator(1,""); //get
    addon.opeator(1,last + result + new Date().getTime().toString()) //set 
} else {
    addon.opeator(0,new Date().getTime().toString());// set
}
