/// author: Goitseone Themba
//

import * as fs from 'fs';
import { promisify } from 'util';

const readFile = promisify(fs.readFile);
const writeFile = promisify(fs.writeFile);

interface Customer {
  id: number;
  name: string;
  status: 'waiting' | 'served';
}

class Custy {
  private dataFile: string = 'customer.json';

  private async readCustomers(): Promise<Customer[]> {
    try {
      const data = await readFile(this.dataFile, 'utf8');
      return JSON.parse(data);
    } catch (error) {
      return [];
    }
  }

  private async writeCustomers(customers: Customer[]): Promise<void> {
    await writeFile(this.dataFile, JSON.stringify(customers, null, 2));
  }

  async add(name: string): Promise<void> {
    const customers = await this.readCustomers();
    const newId = customers.length > 0 ? Math.max(...customers.map(c => c.id)) + 1 : 1;
    const newCustomer: Customer = { id: newId, name, status: 'waiting' };
    
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
