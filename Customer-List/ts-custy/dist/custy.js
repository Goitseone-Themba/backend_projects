"use strict";
/// author: Goitseone Themba
//
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
const fs = __importStar(require("fs"));
const util_1 = require("util");
const readFile = (0, util_1.promisify)(fs.readFile);
const writeFile = (0, util_1.promisify)(fs.writeFile);
class Custy {
    constructor() {
        this.dataFile = 'customer.json';
    }
    async readCustomers() {
        try {
            const data = await readFile(this.dataFile, 'utf8');
            return JSON.parse(data);
        }
        catch (error) {
            return [];
        }
    }
    async writeCustomers(customers) {
        await writeFile(this.dataFile, JSON.stringify(customers, null, 2));
    }
    async add(name) {
        const customers = await this.readCustomers();
        const newId = customers.length > 0 ? Math.max(...customers.map(c => c.id)) + 1 : 1;
        const newCustomer = { id: newId, name, status: 'waiting' };
        customers.push(newCustomer);
        await this.writeCustomers(customers);
        console.log(`${name} added successfully, id: ${newId.toString().padStart(2, '0')}`);
    }
}
async function main() {
    const custy = new Custy();
    const command = process.argv[2];
    const arg = process.argv[3];
    switch (command) {
        case 'add':
            if (arg) {
                await custy.add(arg);
            }
            else {
                console.log('Please provide a name for the new customer.');
            }
            break;
        default:
            console.log('Unknown command');
    }
}
main().catch(console.error);
